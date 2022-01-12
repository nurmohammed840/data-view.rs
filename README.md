[Docs](https://docs.rs/data-view)

This library provides a data view for reading and writing data in a byte array.

This library requires [feature(generic_const_exprs)](https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html) to be enabled. whice is a nightly feature.
So you need nightly compiler to use this library.

It also works with `[no_std]` environment.

By default, this library uses little endian as the default endian.
But you can override the endian by using `BE` (for big endian) or `NE` (for native endian) in fetures flag.

For example, if you want to use big endian,  

```toml
data-view = { version = "2", features = ["BE"] }
```

# Examples
```rust
use data_view::View;

let mut buf: [u8; 16] = [0; 16];

buf.write_at(42_u16, 0);
buf.write_at(123_u32, 2);

assert_eq!(buf.read_at::<u16>(0), 42);
assert_eq!(buf.read_at::<u32>(2), 123);
```
