#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod font;
mod glyph;
#[cfg(all(feature = "libm-math", not(feature = "std")))]
mod nostd_float;
mod outlined;
mod scale;
mod ttfp;

pub use crate::{font::*, glyph::*, outlined::*, scale::*};

pub mod ttf_parser {
    pub use crate::ttfp::*;
    pub use ttf_parser::*;
}
