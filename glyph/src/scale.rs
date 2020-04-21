use crate::{Font, Glyph, GlyphId, OutlinedGlyph};

/// Pixel scale.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct PxScale {
    /// Horizontal scale in pixels.
    pub x: f32,
    /// Vertical scale in pixels.
    pub y: f32,
}

impl From<f32> for PxScale {
    /// Uniform scaling where x & y are the same.
    #[inline]
    fn from(s: f32) -> Self {
        PxScale { x: s, y: s }
    }
}

/// A [`Font`](trait.Font.html) with an associated pixel scale.
pub trait ScaleFont<F: Font> {
    fn scale(&self) -> PxScale;

    fn font(&self) -> &F;

    /// Scale factor for unscaled font horizontal values.
    #[inline]
    fn h_factor(&self) -> f32 {
        self.scale().x / self.font().height()
    }

    /// Scale factor for unscaled font vertical values.
    #[inline]
    fn v_factor(&self) -> f32 {
        self.scale().y / self.font().height()
    }

    /// Pixel scaled glyph ascent.
    #[inline]
    fn ascent(&self) -> f32 {
        self.v_factor() * self.font().ascent()
    }

    /// Pixel scaled glyph descent.
    #[inline]
    fn descent(&self) -> f32 {
        self.v_factor() * self.font().descent()
    }

    /// Pixel scaled height `ascent - descent`.
    #[inline]
    fn height(&self) -> f32 {
        self.scale().y
    }

    /// Pixel scaled line gap.
    #[inline]
    fn line_gap(&self) -> f32 {
        self.v_factor() * self.font().line_gap()
    }

    /// Lookup a `GlyphId` matching a given `char`.
    #[inline]
    fn glyph_id(&self, c: char) -> GlyphId {
        self.font().glyph_id(c)
    }

    /// Convenience method for `font.glyph_id(c).scaled(scale)` using the font scale.
    #[inline]
    fn glyph(&self, c: char) -> Glyph {
        self.font().glyph_id(c).scaled(self.scale())
    }

    /// Pixel scaled horizontal advance for a given glyph.
    #[inline]
    fn h_advance(&self, id: GlyphId) -> f32 {
        self.h_factor() * self.font().h_advance(id)
    }

    /// Pixel scaled horizontal side bearing for a given glyph.
    #[inline]
    fn h_side_bearing(&self, id: GlyphId) -> f32 {
        self.h_factor() * self.font().h_side_bearing(id)
    }

    /// Returns additional pixel scaled kerning to apply for a particular pair of glyphs.
    #[inline]
    fn kern(&self, first: GlyphId, second: GlyphId) -> f32 {
        self.h_factor() * self.font().kern(first, second)
    }

    /// Compute glyph pixel-scaled outline curves & pixel bounding box.
    #[inline]
    fn outline(&self, glyph: Glyph) -> Option<OutlinedGlyph> {
        self.font().outline(glyph)
    }

    #[inline]
    fn as_scaled(&self, scale: PxScale) -> PxScaleFontRef<'_, F>
    where
        Self: core::marker::Sized,
    {
        PxScaleFontRef {
            font: self.font(),
            scale,
        }
    }
}

/// A [`Font`](trait.Font.html) reference and an associated pixel scale.
#[derive(Copy, Clone, Debug)]
pub struct PxScaleFontRef<'a, F> {
    pub font: &'a F,
    pub scale: PxScale,
}

impl<F: Font> ScaleFont<F> for PxScaleFontRef<'_, F> {
    #[inline]
    fn scale(&self) -> PxScale {
        self.scale
    }

    #[inline]
    fn font(&self) -> &F {
        self.font
    }
}

/// A [`Font`](trait.Font.html) and an associated pixel scale.
#[derive(Clone, Debug)]
pub struct PxScaleFont<F> {
    pub font: F,
    pub scale: PxScale,
}

impl<F: Font> ScaleFont<F> for PxScaleFont<F> {
    #[inline]
    fn scale(&self) -> PxScale {
        self.scale
    }

    #[inline]
    fn font(&self) -> &F {
        &self.font
    }
}

impl<F: Font> PxScaleFont<F> {
    #[inline]
    pub fn with_scale(mut self, scale: PxScale) -> PxScaleFont<F> {
        self.scale = scale;
        self
    }
}

impl<'a, F: Font> From<&'a PxScaleFont<F>> for PxScaleFontRef<'a, F> {
    #[inline]
    fn from(sf: &'a PxScaleFont<F>) -> Self {
        Self {
            font: &sf.font,
            scale: sf.scale,
        }
    }
}
