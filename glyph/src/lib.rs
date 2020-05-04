#![warn(missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod err;
mod font;
#[cfg(feature = "std")]
mod font_arc;
mod glyph;
#[cfg(all(feature = "libm-math", not(feature = "std")))]
mod nostd_float;
mod outlined;
mod scale;
mod ttfp;

#[cfg(feature = "std")]
pub use crate::font_arc::*;
pub use crate::{
    err::*,
    font::*,
    glyph::*,
    outlined::*,
    scale::*,
    ttfp::{FontRef, FontVec},
};
