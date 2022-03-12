use crate::endian::*;

/// A data view for reading and writing data in byte array.
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

    // /// Reads a value of type `E: Endian` from view, without doing bounds checking.
    // /// For a safe alternative see [`read_at`].
    // ///
    // /// [`read_at`]: #method.read_at
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use data_view::View;
    // ///
    // /// let mut buf: [u8; 2] = [12, 34];
    // /// unsafe {
    // ///     assert_eq!(buf.read_at_unchecked::<u8>(0), 12);
    // ///     assert_eq!(buf.read_at_unchecked::<u8>(1), 34);
    // /// }
    // /// ```
    // /// # Safety
    // ///
    // /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    // unsafe fn read_at_unchecked<E: Endian>(&self, offset: usize) -> E;

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
    fn write_at<E: Endian>(&mut self, offset: usize, num: E);
}

impl View for [u8] {
    #[inline]
    fn read_at<E: Endian>(&self, offset: usize) -> Option<E> {
        let bytes = self.get(offset..offset + E::SIZE)?;
        Some(unsafe { num_from(bytes.as_ptr()) })
    }
    
    // #[inline]
    // unsafe fn read_at_unchecked<E: Endian>(&self, offset: usize) -> E {
    //     let total_len = offset + E::SIZE;
    //     debug_assert!(total_len <= self.len());
    //     num_from(self.get_unchecked(offset..total_len).as_ptr())
    // }

    #[inline]
    fn write_at<E: Endian>(&mut self, offset: usize, num: E) {
        assert!(offset + E::SIZE <= self.len());
        unsafe { num_write_at(num, self.as_mut_ptr().add(offset)) };
    }
}
