#![allow(clippy::missing_safety_doc)]

use super::*;
use core::fmt::Debug;

/// This trait contains unsafe methods for efficiently reading and writing data.
///
/// Those Methods are unsafe because they do not check the index bounds.
///
/// And safely used by internal. And shouldn't expect to be used by user.
pub trait Endian: Copy + Default + Debug + PartialEq + PartialOrd + Sized + Send + Sync + Unpin {
    unsafe fn __write_at__(self, dst: *mut u8);
    unsafe fn __read_at__(src: *const u8) -> Self;
}

macro_rules! impl_endian_for {
    [$($rty:ty)*] => ($(
        // impl Endian for $rty {}
        impl Endian for $rty {
            unsafe fn __write_at__(self, dst: *mut u8) {
                #[cfg(all(target_endian = "big", not(any(feature = "BE", feature = "NE"))))]
                return write_unaligned(self.to_le_bytes().as_ptr(), dst, size_of::<$rty>());
                #[cfg(all(target_endian = "little", feature = "BE"))]
                return write_unaligned(self.to_be_bytes().as_ptr(), dst, size_of::<$rty>());
                #[cfg(any(
                    feature = "NE",
                    all(target_endian = "big", feature = "BE"),
                    all(target_endian = "little", not(any(feature = "BE", feature = "NE"))),
                ))]
                return write_unaligned(&self as *const Self as *const u8, dst, size_of::<$rty>());
            }
            unsafe fn __read_at__(src: *const u8) -> Self {
                #[cfg(all(target_endian = "big", not(any(feature = "BE", feature = "NE"))))]
                return Self::from_le_bytes(read_unaligned(src));
                #[cfg(all(target_endian = "little", feature = "BE"))]
                return Self::from_be_bytes(read_unaligned(src));
                #[cfg(any(
                    feature = "NE",
                    all(target_endian = "big", feature = "BE"),
                    all(target_endian = "little", not(any(feature = "BE", feature = "NE"))),
                ))]
                return read_unaligned(src);
            }
        }
    )*);
}

impl_endian_for!(
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    usize isize
    f32 f64
);

unsafe fn read_unaligned<T>(src: *const u8) -> T {
    let mut tmp = core::mem::MaybeUninit::<T>::uninit();
    ptr::copy_nonoverlapping(src, tmp.as_mut_ptr() as *mut u8, size_of::<T>());
    tmp.assume_init()
}
unsafe fn write_unaligned(src: *const u8, dst: *mut u8, count: usize) {
    ptr::copy_nonoverlapping(src, dst, count);
}
