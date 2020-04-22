#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod err;
mod font;
mod glyph;
#[cfg(all(feature = "libm-math", not(feature = "std")))]
mod nostd_float;
mod outlined;
mod scale;
mod ttfp;

pub use crate::{
    err::*,
    font::*,
    glyph::*,
    outlined::*,
    scale::*,
    ttfp::{FontRef, FontVec},
};
