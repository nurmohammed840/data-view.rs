pub trait Endian {
    /// The number of bytes.
    const NBYTES: usize;
    /// Return the memory representation of this integer as a byte array in little-endian byte order.
    fn to_bytes_le(self) -> [u8; Self::NBYTES];
    /// Return the memory representation of this integer as a byte array in big-endian (network) byte order.
    fn to_bytes_be(self) -> [u8; Self::NBYTES];
    /// Return the memory representation of this integer as a byte array in native byte order. As the target platform's native endianness is used, portable code should use `to_bytes_le` or `to_bytes_be`, as appropriate, instead.
    fn to_bytes_ne(self) -> [u8; Self::NBYTES];
    /// Create a native endian integer value from its representation as a byte array in little endian.
    fn from_bytes_le(bytes: [u8; Self::NBYTES]) -> Self;
    /// Create a native endian integer value from its representation as a byte array in big endian.
    fn from_bytes_be(bytes: [u8; Self::NBYTES]) -> Self;
    /// Create a native endian integer value from its memory representation as a byte array in native endianness. As the target platform's native endianness is used, portable code likely wants to use `from_bytes_le` or `from_bytes_be`, as appropriate instead.
    fn from_bytes_ne(bytes: [u8; Self::NBYTES]) -> Self;
}
macro_rules! impl_endian_ext {
    [$($rty:ty : $nbyte:literal)*] => ($(
        impl Endian for $rty {
            const NBYTES: usize = $nbyte;
            #[inline(always)]
            fn to_bytes_le(self) -> [u8; Self::NBYTES] { self.to_le_bytes() }
            #[inline(always)]
            fn to_bytes_be(self) -> [u8; Self::NBYTES] { self.to_be_bytes() }
            #[inline(always)]
            fn to_bytes_ne(self) -> [u8; Self::NBYTES] { self.to_ne_bytes() }
            #[inline(always)]
            fn from_bytes_le(bytes: [u8; Self::NBYTES]) -> Self { Self::from_le_bytes(bytes) }
            #[inline(always)]
            fn from_bytes_be(bytes: [u8; Self::NBYTES]) -> Self { Self::from_be_bytes(bytes) }
            #[inline(always)]
            fn from_bytes_ne(bytes: [u8; Self::NBYTES]) -> Self { Self::from_ne_bytes(bytes) }
        }
    )*);
}
impl_endian_ext!(u8:1 u16:2 u32:4 u64:8 u128:16 i8:1 i16:2 i32:4 i64:8 i128:16 f32:4 f64:8);