//! ttf-parser crate specific code. ttf-parser types should not be leaked publicly.
mod outliner;
#[cfg(feature = "variable-fonts")]
mod variable;

use crate::{point, Font, GlyphId, InvalidFont, Outline, Point, Rect};
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt;
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
#[non_exhaustive]
pub struct GlyphImage<'a> {
    /// Offset of the image from the normal origin (top at the baseline plus
    /// ascent), measured in pixels at the image's current scale.
    pub origin: Point,
    /// Image width.
    ///
    /// It doesn't guarantee that this value is the same as set in the `data` in the case of
    /// [`GlyphImageFormat::Png`] format.
    pub width: usize,
    /// Image height.
    ///
    /// It doesn't guarantee that this value is the same as set in the `data` in the case of
    /// [`GlyphImageFormat::Png`] format.
    pub height: usize,
    /// Current scale of the image in pixels per em.
    pub scale: f32,
    /// Raw image data, not a bitmap in the case of [`GlyphImageFormat::Png`] format.
    pub data: &'a [u8],
    /// Format of the raw data.
    pub format: GlyphImageFormat,
}

impl<'a> From<ttfp::RasterGlyphImage<'a>> for GlyphImage<'a> {
    fn from(img: ttfp::RasterGlyphImage<'a>) -> Self {
        GlyphImage {
            origin: point(img.x.into(), img.y.into()),
            width: img.width.into(),
            height: img.height.into(),
            scale: img.pixels_per_em.into(),
            data: img.data,
            format: match img.format {
                ttfp::RasterImageFormat::PNG => GlyphImageFormat::Png,
                ttfp::RasterImageFormat::BitmapMono => GlyphImageFormat::BitmapMono,
                ttfp::RasterImageFormat::BitmapMonoPacked => GlyphImageFormat::BitmapMonoPacked,
                ttfp::RasterImageFormat::BitmapGray2 => GlyphImageFormat::BitmapGray2,
                ttfp::RasterImageFormat::BitmapGray2Packed => GlyphImageFormat::BitmapGray2Packed,
                ttfp::RasterImageFormat::BitmapGray4 => GlyphImageFormat::BitmapGray4,
                ttfp::RasterImageFormat::BitmapGray4Packed => GlyphImageFormat::BitmapGray4Packed,
                ttfp::RasterImageFormat::BitmapGray8 => GlyphImageFormat::BitmapGray8,
                ttfp::RasterImageFormat::BitmapPremulBgra32 => GlyphImageFormat::BitmapPremulBgra32,
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

    /// A monochrome bitmap.
    ///
    /// The most significant bit of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. The data for each row is padded to a byte
    /// boundary, so the next row begins with the most significant bit of a new byte. 1 corresponds
    /// to black, and 0 to white.
    BitmapMono,

    /// A packed monochrome bitmap.
    ///
    /// The most significant bit of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. Data is tightly packed with no padding. 1
    /// corresponds to black, and 0 to white.
    BitmapMonoPacked,

    /// A grayscale bitmap with 2 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. The data for each row is padded to a byte
    /// boundary, so the next row begins with the most significant bit of a new byte.
    BitmapGray2,

    /// A packed grayscale bitmap with 2 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. Data is tightly packed with no padding.
    BitmapGray2Packed,

    /// A grayscale bitmap with 4 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. The data for each row is padded to a byte
    /// boundary, so the next row begins with the most significant bit of a new byte.
    BitmapGray4,

    /// A packed grayscale bitmap with 4 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. Data is tightly packed with no padding.
    BitmapGray4Packed,

    /// A grayscale bitmap with 8 bits per pixel.
    ///
    /// The first byte corresponds to the top-left pixel, proceeding through succeeding bytes
    /// moving left to right.
    BitmapGray8,

    /// A color bitmap with 32 bits per pixel.
    ///
    /// The first group of four bytes corresponds to the top-left pixel, proceeding through
    /// succeeding pixels moving left to right. Each byte corresponds to a color channel and the
    /// channels within a pixel are in blue, green, red, alpha order. Color values are
    /// pre-multiplied by the alpha. For example, the color "full-green with half translucency"
    /// is encoded as `\x00\x80\x00\x80`, and not `\x00\xFF\x00\x80`.
    BitmapPremulBgra32,
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
            ttfp::Face::parse(data, index).map_err(|_| InvalidFont)?,
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

    /// Extracts a slice containing the data passed into e.g. [`FontVec::try_from_vec`].
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
    /// let font_data_clone = owned_font_data.clone();
    /// let font = FontVec::try_from_vec(owned_font_data)?;
    /// assert_eq!(font.as_slice(), font_data_clone);
    /// # Ok(()) }
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.face.as_slice()
    }

    /// Unwraps the data passed into e.g. [`FontVec::try_from_vec`].
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # fn main() -> Result<(), InvalidFont> {
    /// # let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
    /// let font_data_clone = owned_font_data.clone();
    /// let font = FontVec::try_from_vec(owned_font_data)?;
    /// assert_eq!(font.into_vec(), font_data_clone);
    /// # Ok(()) }
    /// ```
    pub fn into_vec(self) -> Vec<u8> {
        self.0.face.into_vec()
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
                    .unwrap_or_default()
                    .into()
            }

            #[inline]
            fn h_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .glyph_hor_side_bearing(id.into())
                    .unwrap_or_default()
                    .into()
            }

            #[inline]
            fn v_advance_unscaled(&self, id: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .glyph_ver_advance(id.into())
                    .unwrap_or_default()
                    .into()
            }

            #[inline]
            fn v_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
                self.0
                    .as_face_ref()
                    .glyph_ver_side_bearing(id.into())
                    .unwrap_or_default()
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

            fn codepoint_ids(&self) -> crate::CodepointIdIter<'_> {
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
