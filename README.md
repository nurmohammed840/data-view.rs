[Docs](https://docs.rs/data-view)

This library provides a data view for reading and writing data in a byte array.

It also works with `[no_std]` environment.

By default, this library uses little endian as the default endianness.
But you can override the endianness by using `BE` (for big endian) or `NE` (for native endian) in fetures flag.

For example, if you want to use big endian,  

```toml
[dependencies]
data-view = { version = "5", features = ["BE"] }
```

# Examples

Add this to your project's `Cargo.toml` file.

```toml
[dependencies]
data-view = "5"
```

### [DataView](https://docs.rs/data-view/latest/data_view/struct.DataView.html)

```rust
use data_view::DataView;

let mut view = DataView::new([0; 8]);

view.write(12_u16);
view.write(34_u16);
view.write(5678_u32);

view.offset = 0;

assert_eq!(view.read::<u16>(), Some(12));
assert_eq!(view.read::<u16>(), Some(34));
assert_eq!(view.read::<u32>(), Some(5678));
```

### [View](https://docs.rs/data-view/latest/data_view/trait.View.html)

```rust
use data_view::View;

let mut buf = [0; 8];

buf.write_at(0, 42_u16);
buf.write_at(2, 123_u32);

assert_eq!(buf.read_at::<u16>(0), Some(42));
assert_eq!(buf.read_at::<u32>(2), Some(123));
```

#### Alternative

There are many alternative libraries,
 * [bytes](https://crates.io/crates/bytes)
 * [byteorder](https://github.com/BurntSushi/byteorder) 
 
But I didn't like API of these libraries.
The have a lot of functions for reading and writing data. For example, `read_u16`, `read_u32`, `write_i64`,  And so on... 

Luckily, Rust support Generics function, This is why this library exists.
