use crate::endian::Endian;
use core::convert::TryInto;

/// A data view for reading and writing data in a byte array.
///
/// # Examples
///
/// ```
/// use data_view::View;
///
/// let mut buf = [0; 16];
///
/// buf.write_at(42_u16, 1);
/// assert_eq!(buf.read_at::<u16>(1), 42);
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
    /// assert_eq!(buf.read_at::<u8>(0), 12);
    /// assert_eq!(buf.read_at::<u8>(1), 34);
    /// ```
    ///
    /// # Panics
    /// Panics if the offset is out of bounds.
    fn read_at<E>(&self, offset: usize) -> Option<E>
    where
        E: Endian,
        [(); E::SIZE]:;

    /// Reads a value of type `E: Endian` from view, without doing bounds checking.
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    unsafe fn read_at_unchecked<E>(&self, offset: usize) -> E
    where
        E: Endian,
        [(); E::SIZE]:;

    /// Writes a value of type `E` to the data view. where `E` is a type that implements `Endian`.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::View;
    ///
    /// let mut buf: [u8; 2] = [0; 2];
    ///
    /// buf.write_at(12_u8, 0);
    /// buf.write_at(34_u8, 1);
    /// assert_eq!(buf, [12, 34]);
    /// ```
    ///
    /// # Panics
    /// Panics if the offset is out of bounds.
    fn write_at<E>(&mut self, offset: usize, value: E)
    where
        E: Endian,
        [u8; E::SIZE]:;
}

impl View for [u8] {
    #[inline]
    fn read_at<E>(&self, offset: usize) -> Option<E>
    where
        E: Endian,
        [u8; E::SIZE]:,
    {
        #[cfg(not(any(feature = "BE", feature = "NE")))]
        return Some(E::from_bytes_le(
            self.get(offset..offset + E::SIZE)?.try_into().unwrap(),
        ));
        #[cfg(feature = "BE")]
        return Some(E::from_bytes_be(
            self.get(offset..offset + E::SIZE)?.try_into().unwrap(),
        ));
        #[cfg(feature = "NE")]
        return Some(E::from_bytes_ne(
            self.get(offset..offset + E::SIZE)?.try_into().unwrap(),
        ));
    }

    #[inline]
    fn write_at<E>(&mut self, offset: usize, value: E)
    where
        E: Endian,
        [(); E::SIZE]:,
    {
        #[cfg(not(any(feature = "BE", feature = "NE")))]
        self[offset..offset + E::SIZE].copy_from_slice(&value.to_bytes_le());
        #[cfg(feature = "BE")]
        self[offset..offset + E::SIZE].copy_from_slice(&value.to_bytes_be());
        #[cfg(feature = "NE")]
        self[offset..offset + E::SIZE].copy_from_slice(&value.to_bytes_ne());
    }

    unsafe fn read_at_unchecked<E>(&self, offset: usize) -> E
    where
        E: Endian,
        [(); E::SIZE]:,
    {
        #[cfg(not(any(feature = "BE", feature = "NE")))]
        return E::from_bytes_le(
            self.get_unchecked(offset..offset + E::SIZE)
                .try_into()
                .unwrap(),
        );
        #[cfg(feature = "BE")]
        return E::from_bytes_be(
            self.get_unchecked(offset..offset + E::SIZE)
                .try_into()
                .unwrap(),
        );
        #[cfg(feature = "NE")]
        return E::from_bytes_ne(
            self.get_unchecked(offset..offset + E::SIZE)
                .try_into()
                .unwrap(),
        );
    }
}
