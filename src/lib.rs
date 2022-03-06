#![no_std]
#![doc = include_str!("../README.md")]

mod view;
mod endian;
mod dataview;

pub use view::View;
pub use endian::Endian;
pub use dataview::DataView;
