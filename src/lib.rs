// #![no_std]
#![doc = include_str!("../README.md")]

mod dataview;
mod endian;
mod view;

pub use dataview::DataView;
pub use endian::Endian;
pub use view::View;