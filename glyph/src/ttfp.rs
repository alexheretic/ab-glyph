//! ttf-parser crate specific code. ttf-parser types should not be leaked publicly.
mod outliner;

use crate::{point, Font, GlyphId, InvalidFont, Outline, Rect};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt;
use owned_ttf_parser::AsFontRef;

impl From<GlyphId> for owned_ttf_parser::GlyphId {
    #[inline]
    fn from(id: GlyphId) -> Self {
        Self(id.0)
    }
}

/// Font data handle stored as a `&[u8]` + parsed data.
/// See [`Font`](trait.Font.html) for more methods.
///
/// Also see the owned version [`FontVec`](struct.FontVec.html).
///
/// # Example
/// ```
/// use ab_glyph::{Font, FontRef};
///
/// # fn main() -> Result<(), ab_glyph::InvalidFont> {
/// let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;
///
/// assert_eq!(font.glyph_id('s'), ab_glyph::GlyphId(56));
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct FontRef<'font>(owned_ttf_parser::Font<'font>);

impl fmt::Debug for FontRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FontRef")
    }
}

impl<'font> FontRef<'font> {
    /// Creates an `FontRef` from a byte-slice.
    ///
    /// For font collections see
    /// [`FontRef::try_from_slice_and_index`](#method.try_from_slice_and_index).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_slice(data: &'font [u8]) -> Result<Self, InvalidFont> {
        Self::try_from_slice_and_index(data, 0)
    }

    /// Creates an `FontRef` from byte-slice.
    ///
    /// You can set index for font collections. For simple fonts use `0` or
    /// [`FontRef::try_from_slice`](#method.try_from_slice).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// let font =
    ///     FontRef::try_from_slice_and_index(include_bytes!("../../dev/fonts/Exo2-Light.otf"), 0)?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_slice_and_index(data: &'font [u8], index: u32) -> Result<Self, InvalidFont> {
        Ok(Self(
            owned_ttf_parser::Font::from_data(data, index).ok_or(InvalidFont)?,
        ))
    }
}

/// Font data handle stored in a `Vec<u8>`  + parsed data.
/// See [`Font`](trait.Font.html) for more methods.
///
/// Also see [`FontRef`](struct.FontRef.html).
///
/// # Example
/// ```
/// use ab_glyph::{Font, FontVec};
///
/// # fn main() -> Result<(), ab_glyph::InvalidFont> {
/// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
/// let font = FontVec::try_from_vec_and_index(owned_font_data, 0)?;
///
/// assert_eq!(font.glyph_id('s'), ab_glyph::GlyphId(56));
/// # Ok(()) }
/// ```
pub struct FontVec(owned_ttf_parser::OwnedFont);

impl fmt::Debug for FontVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FontVec")
    }
}

impl FontVec {
    /// Creates an `FontVec` from owned data.
    ///
    /// For font collections see
    /// [`FontVec::try_from_vec_and_index`](#method.try_from_vec_and_index).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
    /// let font = FontVec::try_from_vec(owned_font_data)?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_vec(data: Vec<u8>) -> Result<Self, InvalidFont> {
        Self::try_from_vec_and_index(data, 0)
    }

    /// Creates an `FontVec` from owned data.
    ///
    /// You can set index for font collections. For simple fonts use `0` or
    /// [`FontVec::try_from_vec`](#method.try_from_vec).
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
    /// let font = FontVec::try_from_vec_and_index(owned_font_data, 0)?;
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn try_from_vec_and_index(data: Vec<u8>, index: u32) -> Result<Self, InvalidFont> {
        Ok(Self(
            owned_ttf_parser::OwnedFont::from_vec(data, index).ok_or(InvalidFont)?,
        ))
    }
}

/// Implement `Font` for `Self(AsFontRef)` types.
macro_rules! impl_font {
    ($font:ty) => {
        impl Font for $font {
            #[inline]
            fn ascent_unscaled(&self) -> f32 {
                f32::from(self.0.as_font().ascender())
            }

            #[inline]
            fn descent_unscaled(&self) -> f32 {
                f32::from(self.0.as_font().descender())
            }

            #[inline]
            fn line_gap_unscaled(&self) -> f32 {
                f32::from(self.0.as_font().line_gap())
            }

            #[inline]
            fn glyph_id(&self, c: char) -> GlyphId {
                let index = self.0.as_font().glyph_index(c).map(|id| id.0).unwrap_or(0);
                GlyphId(index)
            }

            #[inline]
            fn h_advance_unscaled(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_font()
                    .glyph_hor_advance(id.into())
                    .expect("Invalid glyph_hor_advance");
                f32::from(advance)
            }

            #[inline]
            fn h_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_font()
                    .glyph_hor_side_bearing(id.into())
                    .expect("Invalid glyph_hor_side_bearing");
                f32::from(advance)
            }

            #[inline]
            fn kern_unscaled(&self, first: GlyphId, second: GlyphId) -> f32 {
                self.0
                    .as_font()
                    .kerning_subtables()
                    .filter(|st| st.is_horizontal() && !st.is_variable())
                    .find_map(|st| st.glyphs_kerning(first.into(), second.into()))
                    .map(f32::from)
                    .unwrap_or_default()
            }

            fn outline(&self, id: GlyphId) -> Option<Outline> {
                let mut outliner = outliner::OutlineCurveBuilder::default();

                let owned_ttf_parser::Rect {
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                } = self.0.as_font().outline_glyph(id.into(), &mut outliner)?;

                let bounds = Rect {
                    min: point(x_min as f32, y_max as f32),
                    max: point(x_max as f32, y_min as f32),
                };

                Some(Outline {
                    bounds,
                    curves: outliner.take_outline(),
                })
            }

            #[inline]
            fn glyph_count(&self) -> usize {
                self.0.as_font().number_of_glyphs() as _
            }
        }
    };
}

impl_font!(FontRef<'_>);
impl_font!(FontVec);
