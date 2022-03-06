#![no_std]

#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]

//! [Docs](https://docs.rs/data-view)
//!
//! This library provides a data view for reading and writing data in a byte array.
//!
//! This library requires [feature(generic_const_exprs)](https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html) to be enabled. whice is a nightly feature.
//! So you need nightly compiler to use this library.
//!
//! It also works with `[no_std]` environment.
//!
//! By default, this library uses little endian as the default endian.
//! But you can override the endian by using `BE` (for big endian) or `NE` (for native endian) in fetures flag.
//!
//! For example, if you want to use big endian,  
//!
//! ```toml
//! data-view = { version = "2", features = ["BE"] }
//! ```
//!
//! # Examples
//!
//! Add this to your project's `Cargo.toml` file.
//!
//! ```toml
//! data-view = { version = "2", features = ["nightly"] }
//! ```

mod dataview;
pub use dataview::DataView;

#[cfg(feature = "nightly")]
mod view;
#[cfg(feature = "nightly")]
mod endian;

#[cfg(feature = "nightly")]
pub use view::View;
#[cfg(feature = "nightly")]
pub use endian::Endian;