use crate::PxScale;

/// An (x, y) coordinate.
pub type Point = ab_glyph_rasterizer::Point;
pub use ab_glyph_rasterizer::point;

/// Glyph id.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlyphId(pub u16);

impl GlyphId {
    /// Construct a `Glyph` with given scale & position.
    #[inline]
    pub fn scaled_and_position<S: Into<PxScale>, P: Into<Point>>(
        self,
        scale: S,
        position: P,
    ) -> Glyph {
        Glyph {
            id: self,
            scale: scale.into(),
            position: position.into(),
        }
    }

    /// Construct a `Glyph` with given scale and position `point(0.0, 0.0)`.
    #[inline]
    pub fn scaled<S: Into<PxScale>>(self, scale: S) -> Glyph {
        self.scaled_and_position(scale, Point::default())
    }
}

/// A glyph with pixel scale & position.
#[derive(Clone, Debug)]
pub struct Glyph {
    pub id: GlyphId,
    pub scale: PxScale,
    pub position: Point,
}
