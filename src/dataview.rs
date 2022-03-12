use super::*;

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
    /// let mut view = DataView::new([1, 2]);
    ///
    /// assert_eq!(view.remaining_slice(), &[1, 2]);
    /// view.offset = 42;
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
    /// let mut view = DataView::new([0; 4]);
    ///
    /// view.write::<u16>(42);
    /// view.offset = 0;
    /// assert_eq!(view.read::<u16>(), Some(42));
    /// assert_eq!(view.read::<u32>(), None);
    /// ```
    #[inline]
    pub fn read<E: Endian>(&mut self) -> Option<E> {
        let data = self.data.as_ref();
        let total_len = self.offset + size_of::<E>();
        if total_len > data.len() {
            return None;
        }
        let num = unsafe { num_from(data.as_ptr().add(self.offset)) };
        self.offset = total_len;
        Some(num)
    }

    /// Read slice from the current offset.
    ///
    /// # Example
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::new([1, 2, 3]);
    ///
    /// assert_eq!(view.read_slice(2), Some([1, 2].as_ref()));
    /// assert_eq!(view.read_slice(3), None);
    /// ```
    #[inline]
    pub fn read_slice(&mut self, len: usize) -> Option<&[u8]> {
        let total_len = self.offset + len;
        let slice = self.data.as_ref().get(self.offset..total_len)?;
        self.offset = total_len;
        Some(slice)
    }

    /// Create a buffer and returns it, from the current offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::new([1, 2, 3]);
    ///
    /// assert_eq!(view.read_buf(), Some([1, 2]));
    /// assert_eq!(view.read_buf::<3>(), None);
    /// ```
    #[inline]
    pub fn read_buf<const N: usize>(&mut self) -> Option<[u8; N]> {
        let data = self.data.as_ref();
        let total_len = self.offset + N;
        if total_len > data.len() {
            return None;
        }
        let buf = unsafe { ptr::read(data.as_ptr().add(self.offset) as *const [u8; N]) };
        self.offset = total_len;
        Some(buf)
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
    /// let mut view = DataView::new([0; 3]);
    ///
    /// assert_eq!(view.write(42_u16), Ok(()));
    /// assert_eq!(view.write(123_u32), Err(()));
    /// ```
    #[inline]
    pub fn write<E: Endian>(&mut self, num: E) -> Result<(), ()> {
        let data = self.data.as_mut();
        let total_len = self.offset + size_of::<E>();
        if total_len > data.len() {
            return Err(());
        }
        unsafe { num_write_at(num, data.as_mut_ptr().add(self.offset)) };
        self.offset = total_len;
        Ok(())
    }

    /// Writes a slice into the data view.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::new([0; 3]);
    ///
    /// assert_eq!(view.write_slice([4, 2]), Ok(()));
    /// assert_eq!(view.write_slice([1, 2, 3]), Err(()));
    /// assert_eq!(view.data, [4, 2, 0]);
    /// ```
    #[inline]
    pub fn write_slice(&mut self, slice: impl AsRef<[u8]>) -> Result<(), ()> {
        let src = slice.as_ref();
        let data = self.data.as_mut();
        let count = src.len();
        let total_len = self.offset + count;
        if total_len > data.len() {
            return Err(());
        }
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), data.as_mut_ptr().add(self.offset), count);
        }
        self.offset = total_len;
        Ok(())
    }
}

impl<T> From<T> for DataView<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}
