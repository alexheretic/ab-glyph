mod outliner;

use core::fmt;
use ab_glyph_rasterizer::point;
use self_cell::self_cell;
use skrifa::{outline::OutlineGlyphFormat, prelude::{LocationRef, Size}, raw::{tables::kern::SubtableKind, TableProvider}, MetadataProvider};

use crate::{skrifa::outliner::SkrifaCurveBuilder, Font, GlyphImageFormat, InvalidFont};

#[repr(transparent)]
pub struct FontRef<'font> {
    font: PreParsedSubtables<'font>,
}

impl fmt::Debug for FontRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FontRef")
    }
}

struct PreParsedSubtables<'font> {
    font: skrifa::FontRef<'font>,
    location: LocationRef<'font>,
    metrics: skrifa::metrics::Metrics,
    glyph_metrics: skrifa::metrics::GlyphMetrics<'font>,
    outline_glyphs: skrifa::outline::OutlineGlyphCollection<'font>,
    hinting_instance: Option<skrifa::outline::HintingInstance>,
    charmap: skrifa::charmap::Charmap<'font>,
}

impl<'font> PreParsedSubtables<'font> {
    fn new(font: skrifa::FontRef<'font>, location: LocationRef<'font>) -> Self {
        let metrics = font.metrics(Size::unscaled(), location);
        let glyph_metrics = font.glyph_metrics(Size::unscaled(), location);
        let outline_glyphs = font.outline_glyphs();
        let charmap = font.charmap();
        let hinting_instance = match outline_glyphs.format() {
            Some(OutlineGlyphFormat::Glyf) | None => None, // this would enable the autohinter
            _ => skrifa::outline::HintingInstance::new(&outline_glyphs, Size::unscaled(), location, skrifa::outline::HintingOptions::default()).ok()
        };
        Self {
            font,
            location,
            metrics,
            glyph_metrics,
            outline_glyphs,
            hinting_instance,
            charmap,
        }
    }

    fn units_per_em(&self) -> Option<f32> {
        Some(self.metrics.units_per_em as f32)
    }

    fn ascent_unscaled(&self) -> f32 {
        self.metrics.ascent
    }

    fn descent_unscaled(&self) -> f32 {
        self.metrics.descent
    }

    fn line_gap_unscaled(&self) -> f32 {
        self.metrics.leading
    }

    fn glyph_id(&self, c: char) -> crate::GlyphId {
        crate::GlyphId(self.charmap.map(c).unwrap_or(skrifa::GlyphId::NOTDEF).to_u32() as u16)
    }

    fn h_advance_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.glyph_metrics.advance_width(id.into()).unwrap_or_default()
    }

    fn h_side_bearing_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.glyph_metrics.left_side_bearing(id.into()).unwrap_or_default()
    }

    fn v_advance_unscaled(&self, id: crate::GlyphId) -> f32 {
        todo!()
    }

    fn v_side_bearing_unscaled(&self, id: crate::GlyphId) -> f32 {
        todo!()
    }

    fn kern_unscaled(&self, first: crate::GlyphId, second: crate::GlyphId) -> f32 {
        let Ok(kern) = self.font.kern() else {
            return 0.0;
        };
        let first = skrifa::GlyphId::from(first);
        let second = skrifa::GlyphId::from(second);
        kern.subtables().find_map(|st| match st.ok()?.kind().ok()? {
            SubtableKind::Format0(table_ref) => table_ref.kerning(first, second),
            SubtableKind::Format1(_) => None,
            SubtableKind::Format2(subtable2) => subtable2.kerning(first, second),
            SubtableKind::Format3(table_ref) => table_ref.kerning(first, second),
        }).unwrap_or_default() as f32
    }

    fn outline(&self, id: crate::GlyphId) -> Option<crate::Outline> {
        let mut pen = SkrifaCurveBuilder::new();
        let draw_settings = match &self.hinting_instance {
            Some(hinting_instance) => skrifa::outline::DrawSettings::hinted(&hinting_instance, false),
            None => skrifa::outline::DrawSettings::unhinted(Size::unscaled(), self.location),
        };
        self.outline_glyphs.get(id.into())?.draw(draw_settings, &mut pen).ok()?;
        if [pen.bounds.min.x, pen.bounds.min.y, pen.bounds.max.x, pen.bounds.max.y].iter().any(|f| !f.is_finite()) {
            return None;
        }
        Some(crate::Outline { bounds: pen.bounds, curves: pen.curve_builder.take_outline() })
    }

    fn glyph_count(&self) -> usize {
        self.font.maxp().ok().map(|maxp| maxp.num_glyphs()).unwrap_or_default() as usize
    }

    fn codepoint_ids(&self) -> crate::CodepointIdIter<'_> {
        crate::CodepointIdIter { inner: Box::new(self.charmap.mappings().filter_map(|(c, id)| Some((crate::GlyphId::from(id), char::try_from(c).ok()?)))) }
    }

    fn glyph_raster_image2(&self, id: crate::GlyphId, pixel_size: u16) -> Option<crate::v2::GlyphImage<'_>> {
        let bitmap_glyph = self.font.bitmap_strikes().glyph_for_size(Size::new(pixel_size as f32), id.into())?;
        let (format, data) = match bitmap_glyph.data {
            skrifa::bitmap::BitmapData::Bgra(data) => (GlyphImageFormat::BitmapPremulBgra32, data),
            skrifa::bitmap::BitmapData::Png(data) => (GlyphImageFormat::Png, data),
            skrifa::bitmap::BitmapData::Mask(mask_data) => (match (mask_data.bpp, mask_data.is_packed) {
                (1, false) => GlyphImageFormat::BitmapMono,
                (1, true) => GlyphImageFormat::BitmapMonoPacked,
                (2, false) => GlyphImageFormat::BitmapGray2,
                (2, true) => GlyphImageFormat::BitmapGray2Packed,
                (4, false) => GlyphImageFormat::BitmapGray4,
                (4, true) => GlyphImageFormat::BitmapGray4Packed,
                (8, _) => GlyphImageFormat::BitmapGray8,
                _ => unreachable!()
            }, mask_data.data),
        };
        Some(crate::v2::GlyphImage {
            origin: point(bitmap_glyph.inner_bearing_x, bitmap_glyph.inner_bearing_y),
            width: bitmap_glyph.width as u16,
            height: bitmap_glyph.height as u16,
            pixels_per_em: bitmap_glyph.ppem_x as u16,
            data,
            format,
        })
    }
}

impl<'font> FontRef<'font> {
    /// Creates an `FontRef` from a byte-slice.
    ///
    /// For font collections see
    /// [`FontRef::try_from_slice_and_index`].
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
    /// [`FontRef::try_from_slice`].
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
        Ok(Self {
            font: PreParsedSubtables::new(skrifa::FontRef::from_index(data, index).map_err(|_| InvalidFont)?, Default::default()) ,
        }
        )
    }
}

impl Font for FontRef<'_> {
    fn units_per_em(&self) -> Option<f32> {
        self.font.units_per_em()
    }

    fn ascent_unscaled(&self) -> f32 {
        self.font.ascent_unscaled()
    }

    fn descent_unscaled(&self) -> f32 {
        self.font.descent_unscaled()
    }

    fn line_gap_unscaled(&self) -> f32 {
        self.font.line_gap_unscaled()
    }

    fn glyph_id(&self, c: char) -> crate::GlyphId {
        self.font.glyph_id(c)
    }

    fn h_advance_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.font.h_advance_unscaled(id)
    }

    fn h_side_bearing_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.font.h_side_bearing_unscaled(id)
    }

    fn v_advance_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.font.v_advance_unscaled(id)
    }

    fn v_side_bearing_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.font.v_side_bearing_unscaled(id)
    }

    fn kern_unscaled(&self, first: crate::GlyphId, second: crate::GlyphId) -> f32 {
        self.font.kern_unscaled(first, second)
    }

    fn outline(&self, id: crate::GlyphId) -> Option<crate::Outline> {
        self.font.outline(id)
    }

    fn glyph_count(&self) -> usize {
        self.font.glyph_count()
    }

    fn codepoint_ids(&self) -> crate::CodepointIdIter<'_> {
        self.font.codepoint_ids()
    }

    fn glyph_raster_image2(&self, id: crate::GlyphId, pixel_size: u16) -> Option<crate::v2::GlyphImage<'_>> {
        self.font.glyph_raster_image2(id, pixel_size)
    }
}

self_cell!(
    struct FontVecCell {
        owner: Vec<u8>,

        #[covariant]
        dependent: PreParsedSubtables,
    }
);

pub struct FontVec {
    inner: FontVecCell,
}

impl fmt::Debug for FontVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FontVec")
    }
}

impl FontVec {
    /// Creates an `FontVec` from owned data.
    ///
    /// For font collections see
    /// [`FontVec::try_from_vec_and_index`].
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
    /// [`FontVec::try_from_vec`].
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
        /*if skrifa::FontRef::from_index(&data, index).is_err() {
            return Err(InvalidFont);
        }
        return Ok(Self {
            data,
            index,
            location: Default::default(),
        })*/
        let inner = FontVecCell::try_new(data, |data| {
            let skrifa_font = skrifa::FontRef::from_index(&data, index).map_err(|_| InvalidFont)?;
            Ok(PreParsedSubtables::new(skrifa_font, Default::default()))
        })?;
        Ok(Self {inner})
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
        self.inner.borrow_owner()
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
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.inner.into_owner()
    }
}

impl From<crate::GlyphId> for skrifa::GlyphId {
    fn from(value: crate::GlyphId) -> Self {
        Self::new(value.0 as u32)
    }
}

impl From<skrifa::GlyphId> for crate::GlyphId {
    fn from(value: skrifa::GlyphId) -> Self {
        Self(value.to_u32() as u16)
    }
}

impl Font for FontVec {
    fn units_per_em(&self) -> Option<f32> {
        self.inner.borrow_dependent().units_per_em()
    }

    fn ascent_unscaled(&self) -> f32 {
        self.inner.borrow_dependent().ascent_unscaled()
    }

    fn descent_unscaled(&self) -> f32 {
        self.inner.borrow_dependent().descent_unscaled()
    }

    fn line_gap_unscaled(&self) -> f32 {
        self.inner.borrow_dependent().line_gap_unscaled()
    }

    fn glyph_id(&self, c: char) -> crate::GlyphId {
        self.inner.borrow_dependent().glyph_id(c)
    }

    fn h_advance_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.inner.borrow_dependent().h_advance_unscaled(id)
    }

    fn h_side_bearing_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.inner.borrow_dependent().h_side_bearing_unscaled(id)
    }

    fn v_advance_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.inner.borrow_dependent().v_advance_unscaled(id)
    }

    fn v_side_bearing_unscaled(&self, id: crate::GlyphId) -> f32 {
        self.inner.borrow_dependent().v_side_bearing_unscaled(id)
    }

    fn kern_unscaled(&self, first: crate::GlyphId, second: crate::GlyphId) -> f32 {
        self.inner.borrow_dependent().kern_unscaled(first, second)
    }

    fn outline(&self, id: crate::GlyphId) -> Option<crate::Outline> {
        self.inner.borrow_dependent().outline(id)
    }

    fn glyph_count(&self) -> usize {
        self.inner.borrow_dependent().glyph_count()
    }

    fn codepoint_ids(&self) -> crate::CodepointIdIter<'_> {
        self.inner.borrow_dependent().codepoint_ids()
    }

    fn glyph_raster_image2(&self, id: crate::GlyphId, pixel_size: u16) -> Option<crate::v2::GlyphImage<'_>> {
        self.inner.borrow_dependent().glyph_raster_image2(id, pixel_size)
    }
}
