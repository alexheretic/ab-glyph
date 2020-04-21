//! ttf-parser crate specific code. ttf-parser types should not be leaked publicly.
mod outliner;

#[cfg(all(feature = "libm-math", not(feature = "std")))]
use crate::nostd_float::FloatExt;
use crate::{point, Font, Glyph, GlyphId, InvalidFont, OutlinedGlyph, Rect, ScaleFont};
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
#[derive(Clone)]
pub struct FontRef<'font>(owned_ttf_parser::Font<'font>);

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
/// assert_eq!(font.descent() as i32, -201);
/// # Ok(()) }
/// ```
pub struct FontVec(owned_ttf_parser::OwnedFont);

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
            fn ascent(&self) -> f32 {
                f32::from(self.0.as_font().ascender())
            }

            #[inline]
            fn descent(&self) -> f32 {
                f32::from(self.0.as_font().descender())
            }

            #[inline]
            fn line_gap(&self) -> f32 {
                f32::from(self.0.as_font().line_gap())
            }

            #[inline]
            fn glyph_id(&self, c: char) -> GlyphId {
                let index = self.0.as_font().glyph_index(c).map(|id| id.0).unwrap_or(0);
                GlyphId(index)
            }

            #[inline]
            fn h_advance(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_font()
                    .glyph_hor_advance(id.into())
                    .expect("Invalid glyph_hor_advance");
                f32::from(advance)
            }

            #[inline]
            fn h_side_bearing(&self, id: GlyphId) -> f32 {
                let advance = self
                    .0
                    .as_font()
                    .glyph_hor_side_bearing(id.into())
                    .expect("Invalid glyph_hor_side_bearing");
                f32::from(advance)
            }

            #[inline]
            fn kern(&self, first: GlyphId, second: GlyphId) -> f32 {
                self.0
                    .as_font()
                    .glyphs_kerning(first.into(), second.into())
                    .map(f32::from)
                    .unwrap_or_default()
            }

            fn outline(&self, glyph: Glyph) -> Option<OutlinedGlyph> {
                let (h_px_factor, v_px_factor) = {
                    let scaled = self.as_scaled(glyph.scale);
                    (scaled.h_factor(), scaled.v_factor())
                };

                let mut outliner = outliner::OutlineCurveBuilder::new(h_px_factor, v_px_factor);
                let ttf_bounds = self
                    .0
                    .as_font()
                    .outline_glyph(glyph.id.into(), &mut outliner)?;

                let position = glyph.position;
                let owned_ttf_parser::Rect {
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                } = ttf_bounds;

                // Use subpixel fraction in floor/ceil rounding to elimate rounding error
                // from identical subpixel positions
                let (x_trunc, x_fract) = (position.x.trunc(), position.x.fract());
                let (y_trunc, y_fract) = (position.y.trunc(), position.y.fract());

                let px_bounds = Rect {
                    min: point(
                        (x_min as f32 * h_px_factor + x_fract).floor() + x_trunc,
                        (-y_max as f32 * v_px_factor + y_fract).floor() + y_trunc,
                    ),
                    max: point(
                        (x_max as f32 * h_px_factor + x_fract).ceil() + x_trunc,
                        (-y_min as f32 * v_px_factor + y_fract).ceil() + y_trunc,
                    ),
                };

                Some(OutlinedGlyph::new(
                    glyph,
                    px_bounds,
                    outliner.take_outline(),
                ))
            }
        }
    };
}

impl_font!(FontRef<'_>);
impl_font!(FontVec);
