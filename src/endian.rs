#![allow(clippy::missing_safety_doc)]
use core::fmt::Debug;
use core::mem::size_of;
use core::ptr;
use core::ptr::copy_nonoverlapping;

/// This trait contains many unsafe methods for efficiently reading and writing data.
///
/// Those Methods are unsafe because they do not check the index bounds.
///
/// Those methods are safely used by internal. And shouldn't expect to be used by user.
/// You almost never have to implement this trait for your own types.
pub trait Endian: Copy + Default + Debug + PartialEq + PartialOrd {
    unsafe fn write_at_le(self, dst: *mut u8);
    unsafe fn write_at_be(self, dst: *mut u8);
    unsafe fn write_at_ne(self, dst: *mut u8);
    unsafe fn from_bytes_le(src: *const u8) -> Self;
    unsafe fn from_bytes_be(src: *const u8) -> Self;
    unsafe fn from_bytes_ne(src: *const u8) -> Self;
}
macro_rules! impl_endian_for {
    [$($rty:ty)*] => ($(
        impl Endian for $rty {
            #[inline]
            unsafe fn write_at_le(self, dst: *mut u8) { copy_nonoverlapping(self.to_le_bytes().as_ptr(), dst, size_of::<Self>()) }
            #[inline]
            unsafe fn write_at_be(self, dst: *mut u8) { copy_nonoverlapping(self.to_be_bytes().as_ptr(), dst, size_of::<Self>()) }
            #[inline]
            unsafe fn write_at_ne(self, dst: *mut u8) { copy_nonoverlapping(&self as *const Self as *const u8, dst, size_of::<Self>()) }
            #[inline]
            unsafe fn from_bytes_le(src: *const u8) -> Self { Self::from_le_bytes(ptr::read(src as *const [u8; size_of::<Self>()])) }
            #[inline]
            unsafe fn from_bytes_be(src: *const u8) -> Self { Self::from_be_bytes(ptr::read(src as *const [u8; size_of::<Self>()])) }
            #[inline]
            unsafe fn from_bytes_ne(src: *const u8) -> Self { ptr::read_unaligned(src as *const Self) }
        }
    )*);
}

impl_endian_for!(
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    usize isize
    f32 f64
);

#[inline]
pub unsafe fn num_from<E: Endian>(src: *const u8) -> E {
    #[cfg(not(any(feature = "BE", feature = "NE")))]
    return E::from_bytes_le(src);
    #[cfg(feature = "BE")]
    return E::from_bytes_be(src);
    #[cfg(feature = "NE")]
    return E::from_bytes_ne(src);
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
