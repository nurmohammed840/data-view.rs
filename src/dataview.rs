use core::convert::TryInto;

use crate::endian::*;

/// This struct represents a data view for reading and writing data in a byte array.
/// When read/write, This increment current offset by the size of the value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataView<T> {
    pub data: T,
    pub offset: usize,
}

impl<T> DataView<T> {
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let view = DataView::new([0; 16]);
    /// ```
    pub const fn new(data: T) -> Self {
        Self { data, offset: 0 }
    }
}

impl<T: AsRef<[u8]>> DataView<T> {
    /// Returns remaining slice from the current offset.
    /// It doesn't change the offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::from([1, 2, 3]);
    ///
    /// assert_eq!(view.remaining_slice(), &[1, 2, 3]);
    /// view.offset = 3;
    /// assert!(view.remaining_slice().is_empty());
    /// ```
    #[inline]
    pub fn remaining_slice(&self) -> &[u8] {
        let data = self.data.as_ref();
        unsafe { data.get_unchecked(self.offset.min(data.len())..) }
    }

    /// Reads a value of type `E: Endian` from the DataView.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::from([0; 4]);
    ///
    /// view.write::<u16>(42);
    /// view.offset = 0;
    /// assert_eq!(view.read::<u16>().unwrap(), 42);
    /// ```
    #[inline]
    pub fn read<E: Endian>(&mut self) -> Option<E> {
        self.read_slice(E::SIZE)
            .map(|bytes| unsafe { num_from(bytes.as_ptr()) })
    }

    /// Reads a value of type `E: Endian` from the DataView, without doing bounds checking.
    /// For a safe alternative see [`read`].
    ///
    /// [`read`]: #method.read
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    #[inline]
    pub unsafe fn read_unchecked<E: Endian>(&mut self) -> E {
        num_from(self.read_slice_unchecked(E::SIZE).as_ptr())
    }

    /// Read slice from the current offset.
    ///
    /// # Example
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::from([1, 2, 3, 4]);
    ///
    /// assert_eq!(view.read_slice(2).unwrap(), &[1, 2]);
    /// assert_eq!(view.read_slice(2).unwrap(), &[3, 4]);
    /// ```
    #[inline]
    pub fn read_slice(&mut self, len: usize) -> Option<&[u8]> {
        let total_len = self.offset + len;
        let slice = self.data.as_ref().get(self.offset..total_len)?;
        self.offset = total_len;
        Some(slice)
    }

    /// Read slice from the current offset, without doing bounds checking.
    /// For a safe alternative see [`read_slice`].
    ///
    /// [`read_slice`]: #method.read_slice
    ///
    /// # Example
    /// ```
    /// use data_view::DataView;
    /// let mut view = DataView::from([1, 2, 3, 4]);
    /// unsafe {
    ///     assert_eq!(view.read_slice_unchecked(2), &[1, 2]);
    ///     assert_eq!(view.read_slice_unchecked(2), &[3, 4]);
    /// }
    /// ```
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    #[inline]
    pub unsafe fn read_slice_unchecked(&mut self, len: usize) -> &[u8] {
        let data = self.data.as_ref();
        let total_len = self.offset + len;
        debug_assert!(total_len <= data.len());

        let slice = data.get_unchecked(self.offset..total_len);
        self.offset = total_len;
        slice
    }

    /// Create a buffer and returns it, from the current offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::from([1, 2, 3, 4]);
    ///
    /// assert_eq!(view.read_buf::<2>().unwrap(), [1, 2]);
    /// assert_eq!(view.read_buf::<2>().unwrap(), [3, 4]);
    /// ```
    #[inline]
    pub fn read_buf<const N: usize>(&mut self) -> Option<[u8; N]> {
        self.read_slice(N)
            .map(|bytes| unsafe { bytes.try_into().unwrap_unchecked() })
    }

    /// Returns a reference to an element or subslice, without doing bounds checking.
    /// For a safe alternative see [`read_buf`].
    ///
    /// [`read_buf`]: #method.read_buf
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    #[inline]
    pub unsafe fn read_buf_unchecked<const N: usize>(&mut self) -> [u8; N] {
        self.read_slice_unchecked(N).try_into().unwrap_unchecked()
    }
}

impl<T: AsMut<[u8]>> DataView<T> {
    /// Writes a value of type `E` to the data view. where `E` is a type that implements `Endian`.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::from([0; 2]);
    ///
    /// view.write::<u8>(12);
    /// view.write::<u8>(34);
    /// assert_eq!(view.data, [12, 34]);
    /// ```
    /// # Panics
    /// Panics if the offset is out of bounds.
    #[inline]
    pub fn write<E: Endian>(&mut self, num: E) {
        let dst = self.data.as_mut();
        let total_len = self.offset + E::SIZE;
        assert!(total_len <= dst.len());
        unsafe { num_write_at(num, dst.as_mut_ptr().add(self.offset)) };
        self.offset = total_len;
    }

    /// Writes a slice into the data view.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::from([0; 4]);
    /// view.write_slice([1, 2]);
    /// view.write_slice([3, 4]);
    /// assert_eq!(view.data, [1, 2, 3, 4]);
    /// ```
    /// # Panics
    /// Panics if the offset is out of bounds.
    #[inline]
    pub fn write_slice(&mut self, slice: impl AsRef<[u8]>) {
        let src = slice.as_ref();
        let dst = self.data.as_mut();
        let count = src.len();
        let total_len = self.offset + count;
        assert!(total_len <= dst.len());
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(self.offset), count);
        }
        self.offset = total_len;
    }
}

impl<T> From<T> for DataView<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}
