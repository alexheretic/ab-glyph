use crate::PxScale;

/// An (x, y) coordinate. `Point { x: f32, y: f32 }`
pub type Point = ab_glyph_rasterizer::Point;
pub use ab_glyph_rasterizer::point;

/// Glyph id.
///
/// # Example
/// ```
/// use ab_glyph::{FontRef, Font, GlyphId};
/// # fn main() -> Result<(), ab_glyph::InvalidFont> {
/// let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;
///
/// let q_id: GlyphId = font.glyph_id('q');
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlyphId(pub u16);

impl GlyphId {
    /// Construct a `Glyph` with given scale & position.
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf")).unwrap();
    /// let glyph = font.glyph_id('z').with_scale_and_position(24.0, point(100.0, 0.0));
    /// ```
    #[inline]
    pub fn with_scale_and_position<S: Into<PxScale>, P: Into<Point>>(
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
    ///
    /// # Example
    /// ```
    /// # use ab_glyph::*;
    /// # let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf")).unwrap();
    /// let glyph = font.glyph_id('w').with_scale(48.0);
    /// ```
    #[inline]
    pub fn with_scale<S: Into<PxScale>>(self, scale: S) -> Glyph {
        self.with_scale_and_position(scale, Point::default())
    }
}

/// A glyph with pixel scale & position.
#[derive(Clone, Debug)]
pub struct Glyph {
    /// Glyph id.
    pub id: GlyphId,
    /// Pixel scale of this glyph.
    pub scale: PxScale,
    /// Position of this glyph.
    pub position: Point,
}
