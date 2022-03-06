[Docs](https://docs.rs/data-view)

This library provides a data view for reading and writing data in a byte array.

It also works with `[no_std]` environment.

By default, this library uses little endian as the default endianness.
But you can override the endianness by using `BE` (for big endian) or `NE` (for native endian) in fetures flag.

For example, if you want to use big endian,  

```toml
data-view = { version = "4", features = ["BE"] }
```

# Examples

Add this to your project's `Cargo.toml` file.

```toml
data-view = "4"
```

### [DataView](https://docs.rs/data-view/latest/data_view/struct.DataView.html)

```rust
use data_view::DataView;

let mut view = DataView::from([0; 8]);

view.write::<u16>(12);
view.write::<u16>(34);
view.write::<u32>(5678);

view.offset = 0;

assert_eq!(view.read::<u16>().unwrap(), 12);
assert_eq!(view.read::<u16>().unwrap(), 34);
assert_eq!(view.read::<u32>().unwrap(), 5678);
```

### [View](https://docs.rs/data-view/latest/data_view/trait.View.html)

```rust
use data_view::View;

let mut buf: [u8; 16] = [0; 16];

buf.write_at(0, 42_u16);
buf.write_at(2, 123_u32);

assert_eq!(buf.read_at::<u16>(0).unwrap(), 42);
assert_eq!(buf.read_at::<u32>(2).unwrap(), 123);
```

#### Alternative

There are many alternative libraries for example, 
 * [byteorder](https://github.com/BurntSushi/byteorder) or 
 * [bytes](https://crates.io/crates/bytes)
 
But I didn't like API of these libraries.
The have a lot of functions for reading and writing data. For example, `read_u16`, `read_u32`, `write_i64`,  And so on... 

Luckily, Rust support Generics function, This is why this library exists.
