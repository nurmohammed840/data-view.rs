#![allow(clippy::missing_safety_doc)]

use core::convert::TryInto;
use core::fmt::Debug;
use core::ptr::copy_nonoverlapping;

/// This trait contains many unsafe methods for efficiently reading and writing data.
///
/// Those Methods are unsafe because they do not check the index bounds.
///
/// Those methods are safely used by internal. And shouldn't expect to be used by user.
/// You almost never have to implement this trait for your own types.
pub trait Endian: Copy + Default + Debug + PartialEq + PartialOrd {
    const SIZE: usize;
    unsafe fn write_at_le(self, dst: *mut u8);
    unsafe fn write_at_be(self, dst: *mut u8);
    unsafe fn write_at_ne(self, dst: *mut u8);
    unsafe fn from_bytes_le(bytes: &[u8]) -> Self;
    unsafe fn from_bytes_be(bytes: &[u8]) -> Self;
    unsafe fn from_bytes_ne(bytes: &[u8]) -> Self;
}
macro_rules! impl_endian_for {
    [$($rty:ty : $size:literal)*] => ($(
        impl Endian for $rty {
            const SIZE: usize = $size;
            #[inline]
            unsafe fn write_at_le(self, dst: *mut u8) { copy_nonoverlapping(self.to_le_bytes().as_ptr(), dst, $size) }
            #[inline]
            unsafe fn write_at_be(self, dst: *mut u8) { copy_nonoverlapping(self.to_be_bytes().as_ptr(), dst, $size) }
            #[inline]
            unsafe fn write_at_ne(self, dst: *mut u8) { copy_nonoverlapping(self.to_ne_bytes().as_ptr(), dst, $size) }
            #[inline]
            unsafe fn from_bytes_le(bytes: &[u8]) -> Self { Self::from_le_bytes(bytes.try_into().unwrap_unchecked()) }
            #[inline]
            unsafe fn from_bytes_be(bytes: &[u8]) -> Self { Self::from_be_bytes(bytes.try_into().unwrap_unchecked()) }
            #[inline]
            unsafe fn from_bytes_ne(bytes: &[u8]) -> Self { Self::from_ne_bytes(bytes.try_into().unwrap_unchecked()) }
        }
    )*);
}

#[cfg(target_pointer_width = "16")]
impl_endian_for!(usize:2 isize:2);
#[cfg(target_pointer_width = "32")]
impl_endian_for!(usize:4 isize:4);
#[cfg(target_pointer_width = "64")]
impl_endian_for!(usize:8 isize:8);

impl_endian_for!(
    u8:1 u16:2 u32:4 u64:8 u128:16
    i8:1 i16:2 i32:4 i64:8 i128:16
    f32:4 f64:8
);

#[inline]
pub unsafe fn num_from<E: Endian>(bytes: &[u8]) -> E {
    #[cfg(not(any(feature = "BE", feature = "NE")))]
    return E::from_bytes_le(bytes);
    #[cfg(feature = "BE")]
    return E::from_bytes_be(bytes);
    #[cfg(feature = "NE")]
    return E::from_bytes_ne(bytes);
}

#[inline]
pub unsafe fn num_write_at<E: Endian>(num: E, dst: *mut u8) {
    #[cfg(not(any(feature = "BE", feature = "NE")))]
    num.write_at_le(dst);
    #[cfg(feature = "BE")]
    num.write_at_be(dst);
    #[cfg(feature = "NE")]
    num.write_at_ne(dst);
}
