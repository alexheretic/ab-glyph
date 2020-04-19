use crate::{glyph::*, outlined::*, scale::*};

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

    /// Unscaled horizontal advance for a given glyph.
    fn h_advance(&self, id: GlyphId) -> f32;

    /// Unscaled horizontal side bearing for a given glyph.
    fn h_side_bearing(&self, id: GlyphId) -> f32;

    /// Returns additional unscaled kerning to apply for a particular pair of glyphs.
    fn kern(&self, first: GlyphId, second: GlyphId) -> f32;

    /// Compute glyph pixel-scaled outline curves & pixel bounding box.
    ///
    /// Note: The outline curves are relative to position `(0, 0)` rather than the
    /// glyph position.
    fn outline(&self, glyph: Glyph) -> Option<OutlinedGlyph>;

    #[inline]
    fn as_scaled(&self, scale: PxScale) -> PxScaleFontRef<'_, Self>
    where
        Self: std::marker::Sized,
    {
        PxScaleFontRef { font: &self, scale }
    }

    #[inline]
    fn into_scaled(self, scale: PxScale) -> PxScaleFont<Self>
    where
        Self: std::marker::Sized,
    {
        PxScaleFont { font: self, scale }
    }
}
