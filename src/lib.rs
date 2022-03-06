#![no_std]

#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]

// #![doc = include_str!("../README.md")]

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