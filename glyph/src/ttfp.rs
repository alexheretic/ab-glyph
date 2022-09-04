//! ttf-parser crate specific code. ttf-parser types should not be leaked publicly.
mod outliner;
#[cfg(feature = "variable-fonts")]
mod variable;

use crate::{point, Font, GlyphId, InvalidFont, Outline, Point, Rect};
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::{convert::TryFrom, fmt};
use owned_ttf_parser::{self as ttfp, AsFaceRef};

impl From<GlyphId> for ttfp::GlyphId {
    #[inline]
    fn from(id: GlyphId) -> Self {
        Self(id.0)
    }
}

/// A pre-rendered image of a glyph, usually used for emojis or other glyphs
/// that can't be represented only using an outline.
#[derive(Debug, Clone)]
pub struct GlyphImage<'a> {
    /// Offset of the image from the normal origin (top at the baseline plus
    /// ascent), measured in pixels at the image's current scale.
    pub origin: Point,
    /// Current scale of the image in pixels per em.
    pub scale: f32,
    /// Raw image data (not a bitmap).
    pub data: &'a [u8],
    /// Format of the raw data.
    pub format: GlyphImageFormat,
}

impl<'a> From<ttfp::RasterGlyphImage<'a>> for GlyphImage<'a> {
    fn from(img: ttfp::RasterGlyphImage<'a>) -> Self {
        GlyphImage {
            origin: point(img.x.into(), img.y.into()),
            scale: img.pixels_per_em.into(),
            data: img.data,
            format: match img.format {
                ttfp::RasterImageFormat::PNG => GlyphImageFormat::Png,
            },
        }
    }
}

/// Valid formats for a [`GlyphImage`].
// Possible future formats:  SVG, JPEG, TIFF
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum GlyphImageFormat {
    Png,
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
pub struct FontRef<'font>(ttfp::PreParsedSubtables<'font, ttfp::Face<'font>>);

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
        Ok(Self(ttfp::PreParsedSubtables::from(
            ttfp::Face::from_slice(data, index).map_err(|_| InvalidFont)?,
        )))
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
pub struct FontVec(ttfp::PreParsedSubtables<'static, ttfp::OwnedFace>);

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
        Ok(Self(ttfp::PreParsedSubtables::from(
            ttfp::OwnedFace::from_vec(data, index).map_err(|_| InvalidFont)?,
        )))
    }
}

/// Implement `Font` for `Self(AsFontRef)` types.
macro_rules! impl_font {
    ($font:ty) => {
        impl Font for $font {
            #[inline]
            fn units_per_em(&self) -> Option<f32> {
                // TODO unwrap signature when making next breaking change
                Some(self.0.as_face_ref().units_per_em().into())
            }

            #[inline]
            fn ascent_unscaled(&self) -> f32 {
                self.0.as_face_ref().ascender().into()
            }

            #[inline]
            fn descent_unscaled(&self) -> f32 {
                self.0.as_face_ref().descender().into()
            }

            #[inline]
            fn line_gap_unscaled(&self) -> f32 {
                self.0.as_face_ref().line_gap().into()
            }

            #[inline]
            fn glyph_id(&self, c: char) -> GlyphId {
                // Note: Using `PreParsedSubtables` method for better performance.
                let index = self.0.glyph_index(c).map(|id| id.0).unwrap_or(0);
                GlyphId(index)
            }

            #[inline]
            fn h_advance_unscaled(&self, id: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .glyph_hor_advance(id.into())
                    .expect("Invalid glyph_hor_advance")
                    .into()
            }

            #[inline]
            fn h_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .glyph_hor_side_bearing(id.into())
                    .expect("Invalid glyph_hor_side_bearing")
                    .into()
            }

            #[inline]
            fn v_advance_unscaled(&self, id: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .glyph_ver_advance(id.into())
                    .expect("Invalid glyph_ver_advance")
                    .into()
            }

            #[inline]
            fn v_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .glyph_ver_side_bearing(id.into())
                    .expect("Invalid glyph_ver_side_bearing")
                    .into()
            }

            #[inline]
            fn kern_unscaled(&self, first: GlyphId, second: GlyphId) -> f32 {
                // Note: Using `PreParsedSubtables` method for better performance.
                self.0
                    .glyphs_hor_kerning(first.into(), second.into())
                    .map(f32::from)
                    .unwrap_or_default()
            }

            fn outline(&self, id: GlyphId) -> Option<Outline> {
                let mut outliner = outliner::OutlineCurveBuilder::default();

                let ttfp::Rect {
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                } = self
                    .0
                    .as_face_ref()
                    .outline_glyph(id.into(), &mut outliner)
                    // invalid bounds are treated as having no outline
                    .filter(|b| b.x_min < b.x_max && b.y_min < b.y_max)?;

                let curves = outliner.take_outline();

                let bounds = Rect {
                    min: point(x_min.into(), y_max.into()),
                    max: point(x_max.into(), y_min.into()),
                };

                Some(Outline { bounds, curves })
            }

            #[inline]
            fn glyph_count(&self) -> usize {
                self.0.as_face_ref().number_of_glyphs() as _
            }

            fn codepoint_ids<'a>(&'a self) -> crate::CodepointIdIter<'a> {
                let face_ref = self.0.as_face_ref();

                #[cfg(feature = "std")]
                let mut used_indices =
                    std::collections::HashSet::with_capacity(face_ref.number_of_glyphs() as _);
                #[cfg(not(feature = "std"))]
                let mut used_indices = alloc::collections::BTreeSet::new();

                let inner = Box::new(
                    face_ref
                        .tables()
                        .cmap
                        .iter()
                        .flat_map(|c| c.subtables)
                        .filter(|s| s.is_unicode())
                        .flat_map(move |subtable| {
                            let mut pairs = Vec::new();
                            subtable.codepoints(|c| {
                                if let Ok(ch) = char::try_from(c) {
                                    if let Some(idx) = subtable.glyph_index(c).filter(|i| i.0 > 0) {
                                        if used_indices.insert(idx.0) {
                                            pairs.push((GlyphId(idx.0), ch));
                                        }
                                    }
                                }
                            });
                            pairs
                        }),
                );

                crate::CodepointIdIter { inner }
            }

            fn glyph_raster_image(&self, id: GlyphId, size: u16) -> Option<GlyphImage> {
                self.0
                    .as_face_ref()
                    .glyph_raster_image(id.into(), size)
                    .map(Into::into)
            }
        }
    };
}

impl_font!(FontRef<'_>);
impl_font!(FontVec);
