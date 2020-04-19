extern crate alloc;

mod font;
mod glyph;
mod outlined;
mod scale;
mod ttfp;

pub use crate::{font::*, glyph::*, outlined::*, scale::*};

pub mod ttf_parser {
    pub use crate::ttfp::*;
    pub use ttf_parser::*;
}
