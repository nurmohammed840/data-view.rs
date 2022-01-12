#![doc = include_str!("../README.md")]

#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod dataview;
mod endian;
mod view;

pub use dataview::DataView;
pub use endian::Endian;
pub use view::View;
