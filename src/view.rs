use core::mem::size_of;

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
    /// let buf = [42];
    ///
    /// assert_eq!(buf.read_at::<u8>(0), Some(42));
    /// assert_eq!(buf.read_at::<u16>(1), None);
    /// ```
    fn read_at<E: Endian>(&self, offset: usize) -> Option<E>;

    /// Writes a value of type `E: Endian` to data view.
    ///
    /// # Examples
    ///
    /// ```
    /// use data_view::View;
    ///
    /// let mut buf = [0];
    ///
    /// assert_eq!(buf.write_at(0, 42_u8), Ok(()));
    /// assert_eq!(buf.write_at(1, 123_u16), Err(()));
    /// ```
    ///
    /// # Panics
    /// Panics if the offset is out of bounds.
    fn write_at<E: Endian>(&mut self, offset: usize, num: E) -> Result<(), ()>;
}

impl View for [u8] {
    #[inline]
    fn read_at<E: Endian>(&self, offset: usize) -> Option<E> {
        if offset + size_of::<E>() > self.len() {
            return None;
        }
        Some(unsafe { num_from(self.as_ptr().add(offset)) })
    }

    #[inline]
    fn write_at<E: Endian>(&mut self, offset: usize, num: E) -> Result<(), ()> {
        if offset + size_of::<E>() > self.len() {
            return Err(());
        }
        Ok(unsafe { num_write_at(num, self.as_mut_ptr().add(offset)) })
    }
}