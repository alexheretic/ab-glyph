use crate::{Glyph, GlyphId, OutlinedGlyph, PxScale, PxScaleFont};

/// Functionality required from font data.
///
/// See [`FontRef`](struct.FontRef.html) & [`FontVec`](struct.FontVec.html).
pub trait Font {
    /// Unscaled glyph ascent.
    fn ascent(&self) -> f32;

    /// Unscaled glyph descent.
    fn descent(&self) -> f32;

    /// Unscaled height `ascent - descent`.
    #[inline]
    fn height(&self) -> f32 {
        self.ascent() - self.descent()
    }

    /// Unscaled line gap.
    fn line_gap(&self) -> f32;

    /// Lookup a `GlyphId` matching a given `char`.
    fn glyph_id(&self, c: char) -> GlyphId;

    /// Unscaled horizontal advance for a given glyph id.
    fn h_advance(&self, id: GlyphId) -> f32;

    /// Unscaled horizontal side bearing for a given glyph id.
    fn h_side_bearing(&self, id: GlyphId) -> f32;

    /// Returns additional unscaled kerning to apply for a particular pair of glyph ids.
    fn kern(&self, first: GlyphId, second: GlyphId) -> f32;

    /// Compute glyph pixel-scaled outline curves & pixel bounding box.
    // Implemtation note: The outline curves are relative to position `(0, 0)` rather than the
    // glyph position.
    fn outline(&self, glyph: Glyph) -> Option<OutlinedGlyph>;

    /// Construct a [`PxScaleFontRef`](struct.PxScaleFontRef.html) by associating with the
    /// given pixel `scale`.
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
    /// # fn main() -> Result<(), ab_glyph::InvalidFont> {
    /// let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;
    ///
    /// // unscaled descent
    /// assert_eq!(font.descent(), -201.0);
    ///
    /// assert_eq!(font.as_scaled(24.0).descent(), -4.02);
    /// assert_eq!(font.as_scaled(50.0).descent(), -8.375);
    /// # Ok(()) }
    /// ```
    #[inline]
    fn as_scaled<S: Into<PxScale>>(&self, scale: S) -> PxScaleFont<&'_ Self>
    where
        Self: core::marker::Sized,
    {
        PxScaleFont {
            font: &self,
            scale: scale.into(),
        }
    }

    /// Move into a [`PxScaleFont`](struct.PxScaleFont.html) associated with the
    /// given pixel `scale`.
    #[inline]
    fn into_scaled<S: Into<PxScale>>(self, scale: S) -> PxScaleFont<Self>
    where
        Self: core::marker::Sized,
    {
        PxScaleFont {
            font: self,
            scale: scale.into(),
        }
    }
}

impl<F: Font> Font for &F {
    #[inline]
    fn ascent(&self) -> f32 {
        (*self).ascent()
    }

    #[inline]
    fn descent(&self) -> f32 {
        (*self).descent()
    }

    #[inline]
    fn line_gap(&self) -> f32 {
        (*self).line_gap()
    }

    #[inline]
    fn glyph_id(&self, c: char) -> GlyphId {
        (*self).glyph_id(c)
    }

    #[inline]
    fn h_advance(&self, id: GlyphId) -> f32 {
        (*self).h_advance(id)
    }

    #[inline]
    fn h_side_bearing(&self, id: GlyphId) -> f32 {
        (*self).h_side_bearing(id)
    }

    #[inline]
    fn kern(&self, first: GlyphId, second: GlyphId) -> f32 {
        (*self).kern(first, second)
    }

    #[inline]
    fn outline(&self, glyph: Glyph) -> Option<OutlinedGlyph> {
        (*self).outline(glyph)
    }
}
