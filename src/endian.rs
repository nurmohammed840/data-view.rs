#![allow(clippy::missing_safety_doc)]

use core::convert::TryInto;
use core::ptr::copy_nonoverlapping;

/// This trait contains many unsafe methods for efficiently reading and writing data.
///
/// Those Methods are unsafe because they do not check the index bounds.
///
/// Those methods are safely used by internal. And shouldn't expect to be used by user.
/// You almost never have to implement this trait for your own types.
pub trait Endian {
    const SIZE: usize;
    unsafe fn bytes_cpy_le(self, dst: *mut u8);
    unsafe fn bytes_cpy_be(self, dst: *mut u8);
    unsafe fn bytes_cpy_ne(self, dst: *mut u8);
    unsafe fn from_bytes_le(bytes: &[u8]) -> Self;
    unsafe fn from_bytes_be(bytes: &[u8]) -> Self;
    unsafe fn from_bytes_ne(bytes: &[u8]) -> Self;
}
macro_rules! impl_endian_ext {
    [$($rty:ty : $size:literal)*] => ($(
        impl Endian for $rty {
            const SIZE: usize = $size;
            #[inline]
            unsafe fn bytes_cpy_le(self, dst: *mut u8) { copy_nonoverlapping(self.to_le_bytes().as_ptr(), dst, $size) }
            #[inline]
            unsafe fn bytes_cpy_be(self, dst: *mut u8) { copy_nonoverlapping(self.to_be_bytes().as_ptr(), dst, $size) }
            #[inline]
            unsafe fn bytes_cpy_ne(self, dst: *mut u8) { copy_nonoverlapping(self.to_ne_bytes().as_ptr(), dst, $size) }
            #[inline]
            unsafe fn from_bytes_le(bytes: &[u8]) -> Self { Self::from_le_bytes(bytes.try_into().unwrap_unchecked()) }
            #[inline]
            unsafe fn from_bytes_be(bytes: &[u8]) -> Self { Self::from_be_bytes(bytes.try_into().unwrap_unchecked()) }
            #[inline]
            unsafe fn from_bytes_ne(bytes: &[u8]) -> Self { Self::from_ne_bytes(bytes.try_into().unwrap_unchecked()) }
        }
    )*);
}
impl_endian_ext!(
    u8:1 u16:2 u32:4 u64:8 u128:16
    i8:1 i16:2 i32:4 i64:8 i128:16
    f32:4 f64:8
);
