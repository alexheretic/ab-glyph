//! **ttf_parser** crate specific code.
mod outliner;
mod owned;

pub use owned::*;

#[cfg(all(feature = "libm-math", not(feature = "std")))]
use crate::nostd_float::FloatExt;

use crate::*;

impl From<GlyphId> for ttf_parser::GlyphId {
    #[inline]
    fn from(id: GlyphId) -> Self {
        Self(id.0)
    }
}

pub trait AsFontRef {
    fn font(&self) -> &ttf_parser::Font<'_>;
}

impl AsFontRef for ttf_parser::Font<'_> {
    #[inline]
    fn font(&self) -> &ttf_parser::Font<'_> {
        self
    }
}

impl AsFontRef for &ttf_parser::Font<'_> {
    #[inline]
    fn font(&self) -> &ttf_parser::Font<'_> {
        self
    }
}

impl<T> Font for T
where
    T: AsFontRef,
{
    #[inline]
    fn ascent(&self) -> f32 {
        f32::from(self.font().ascender())
    }

    #[inline]
    fn descent(&self) -> f32 {
        f32::from(self.font().descender())
    }

    #[inline]
    fn line_gap(&self) -> f32 {
        f32::from(self.font().line_gap())
    }

    #[inline]
    fn glyph_id(&self, c: char) -> GlyphId {
        let index = self.font().glyph_index(c).map(|id| id.0).unwrap_or(0);
        GlyphId(index)
    }

    #[inline]
    fn h_advance(&self, id: GlyphId) -> f32 {
        let advance = self
            .font()
            .glyph_hor_advance(id.into())
            .expect("Invalid glyph_hor_advance");
        f32::from(advance)
    }

    #[inline]
    fn h_side_bearing(&self, id: GlyphId) -> f32 {
        let advance = self
            .font()
            .glyph_hor_side_bearing(id.into())
            .expect("Invalid glyph_hor_side_bearing");
        f32::from(advance)
    }

    #[inline]
    fn kern(&self, first: GlyphId, second: GlyphId) -> f32 {
        self.font()
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
        let ttf_bounds = self.font().outline_glyph(glyph.id.into(), &mut outliner)?;

        let Glyph { position, .. } = glyph;
        let ttf_parser::Rect {
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
