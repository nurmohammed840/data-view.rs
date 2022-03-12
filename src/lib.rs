#![no_std]
#![allow(clippy::result_unit_err)]
#![doc = include_str!("../README.md")]

mod dataview;
mod endian;
mod view;

use core::mem::size_of;
use core::ptr;
use endian::*;

pub use dataview::DataView;
pub use endian::Endian;
pub use view::View;