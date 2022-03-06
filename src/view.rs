use crate::endian::Endian;

/// A data view for reading and writing data in a byte array.
///
/// # Examples
///
/// ```
/// use data_view::View;
///
/// let mut buf = [0; 16];
///
/// buf.write_at(1, 42_u16);
/// assert_eq!(buf.read_at::<u16>(1).unwrap(), 42);
/// ```
///
/// # Panics
/// Panics if the offset is out of bounds.
pub trait View {
    /// Reads a value of type `E: Endian` from view.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::View;
    ///
    /// let mut buf: [u8; 2] = [12, 34];
    ///
    /// assert_eq!(buf.read_at::<u8>(0).unwrap(), 12);
    /// assert_eq!(buf.read_at::<u8>(1).unwrap(), 34);
    /// ```
    fn read_at<E: Endian>(&self, offset: usize) -> Option<E>;

    /// Reads a value of type `E: Endian` from view, without doing bounds checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::View;
    ///
    /// let mut buf: [u8; 2] = [12, 34];
    /// unsafe {
    ///     assert_eq!(buf.read_at_unchecked::<u8>(0), 12);
    ///     assert_eq!(buf.read_at_unchecked::<u8>(1), 34);
    /// }
    /// ```
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    unsafe fn read_at_unchecked<E: Endian>(&self, offset: usize) -> E;

    /// Writes a value of type `E: Endian` to data view.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::View;
    ///
    /// let mut buf: [u8; 2] = [0; 2];
    ///
    /// buf.write_at(0, 12_u8);
    /// buf.write_at(1, 34_u8);
    /// assert_eq!(buf, [12, 34]);
    /// ```
    ///
    /// # Panics
    /// Panics if the offset is out of bounds.
    fn write_at<E: Endian>(&mut self, offset: usize, value: E);
}

impl View for [u8] {
    #[inline]
    fn read_at<E: Endian>(&self, offset: usize) -> Option<E> {
        let bytes = self.get(offset..offset + E::SIZE)?;
        unsafe {
            #[cfg(not(any(feature = "BE", feature = "NE")))]
            return Some(E::from_bytes_le(bytes));
            #[cfg(feature = "BE")]
            return Some(E::from_bytes_be(bytes));
            #[cfg(feature = "NE")]
            return Some(E::from_bytes_ne(bytes));
        }
    }
    #[inline]
    unsafe fn read_at_unchecked<E: Endian>(&self, offset: usize) -> E {
        debug_assert!(offset + E::SIZE <= self.len());
        let bytes = self.get_unchecked(offset..offset + E::SIZE);
        #[cfg(not(any(feature = "BE", feature = "NE")))]
        return E::from_bytes_le(bytes);
        #[cfg(feature = "BE")]
        return E::from_bytes_be(bytes);
        #[cfg(feature = "NE")]
        return E::from_bytes_ne(bytes);
    }
    #[inline]
    fn write_at<E: Endian>(&mut self, offset: usize, value: E) {
        assert!(offset + E::SIZE <= self.len());
        #[cfg(not(any(feature = "BE", feature = "NE")))]
        unsafe { E::bytes_cpy_le(value, self.as_mut_ptr().add(offset)) };
        #[cfg(feature = "BE")]
        unsafe { E::bytes_cpy_be(value, self.as_mut_ptr().add(offset)) };
        #[cfg(feature = "NE")]
        unsafe { E::bytes_cpy_ne(value, self.as_mut_ptr().add(offset)) };
    }
}
