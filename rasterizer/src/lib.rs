#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(all(feature = "libm", not(feature = "std")))]
mod nostd_float;

mod geometry;
mod raster;

pub use geometry::{Point, point};
pub use raster::Rasterizer;
