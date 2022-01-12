use crate::{Endian, View};
use core::convert::TryInto;

/// This struct represents a data view for reading and writing data in a byte array.
/// When read/write, This increment current offset by the size of the value.
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
/// assert_eq!(view.read::<u16>(), 42);
/// ```
#[derive(Debug, Clone)]
pub struct DataView<T> {
    pub data: T,
    pub offset: usize,
}

impl<T: AsRef<[u8]>> DataView<T> {
    /// Creates a new `View` from a byte array.
    /// The offset is set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let view = DataView::new([0; 16]);
    /// ```
    #[inline(always)]
    pub fn new(data: T) -> Self {
        Self { data, offset: 0 }
    }

    /// Returns remaining slice from the current offset. 
    /// It doesn't change the offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::new([1, 2, 3]);
    ///
    /// assert_eq!(view.remaining_slice(), &[1, 2, 3]);
    /// view.offset = 3;
    /// assert!(view.remaining_slice().is_empty());
    /// ```
    #[inline(always)]
    pub fn remaining_slice(&self) -> &[u8] {
        let data = self.data.as_ref();
        &data[self.offset.min(data.len())..]
    }

    /// Reads a value of type `E` from the data view. where `E` implements `Endian`.
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
    /// assert_eq!(view.read::<u16>(), 42);
    /// ```
    /// # Panics
    /// Panics if the offset is out of bounds.
    #[inline(always)]
    pub fn read<E>(&mut self) -> E
    where
        E: Endian,
        [(); E::NBYTES]:,
    {
        let value = self.data.as_ref().read_at(self.offset);
        self.offset += E::NBYTES;
        value
    }

    /// Read slice from the current offset.
    ///
    /// # Example
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::new([1, 2, 3, 4]);
    ///
    /// assert_eq!(view.read_slice(2), &[1, 2]);
    /// assert_eq!(view.read_slice(2), &[3, 4]);
    /// ```
    /// # Panics
    /// Panics if the offset is out of bounds.
    #[inline(always)]
    pub fn read_slice(&mut self, len: usize) -> &[u8] {
        let slice = &self.data.as_ref()[self.offset..self.offset + len];
        self.offset += len;
        slice
    }

    /// Create a buffer and returns it, from the current offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::new([1, 2, 3, 4]);
    ///
    /// assert_eq!(view.read_buf::<2>(), [1, 2]);
    /// assert_eq!(view.read_buf::<2>(), [3, 4]);
    /// ```
    ///
    /// # Panics
    /// Panics if the offset is out of bounds.
    #[inline(always)]
    pub fn read_buf<const N: usize>(&mut self) -> [u8; N] {
        self.read_slice(N).try_into().unwrap()
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
    /// let mut view = DataView::new([0; 2]);
    ///
    /// view.write::<u8>(12);
    /// view.write::<u8>(34);
    /// assert_eq!(view.data, [12, 34]);
    /// ```
    /// # Panics
    /// Panics if the offset is out of bounds.
    #[inline(always)]
    pub fn write<E>(&mut self, value: E)
    where
        E: Endian,
        [(); E::NBYTES]:,
    {
        self.data.as_mut().write_at(value, self.offset);
        self.offset += E::NBYTES;
    }

    /// Writes a slice into the data view.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::DataView;
    ///
    /// let mut view = DataView::new([0; 4]);
    /// view.write_slice([1, 2]);
    /// view.write_slice([3, 4]);
    /// assert_eq!(view.data, [1, 2, 3, 4]);
    /// ```
    /// # Panics
    /// Panics if the offset is out of bounds.
    #[inline(always)]
    pub fn write_slice(&mut self, slice: impl AsRef<[u8]>) {
        let slice = slice.as_ref();
        self.data.as_mut()[self.offset..self.offset + slice.len()].copy_from_slice(slice);
        self.offset += slice.len();
    }
}