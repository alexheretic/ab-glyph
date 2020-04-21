use crate::{Glyph, Point};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// A glyph that has been outlined at a scale & position.
#[derive(Clone, Debug)]
pub struct OutlinedGlyph {
    glyph: Glyph,
    // Pixel scale bounds.
    bounds: Rect,
    // Relatively positioned (from point(0, 0)) pixel-scale outline curves.
    outline: Vec<OutlineCurve>,
}

impl OutlinedGlyph {
    /// Constructs an `OutlinedGlyph` from the source `Glyph`, pixel bounds
    /// & relatively positioned outline curves.
    #[inline]
    pub fn new(glyph: Glyph, bounds: Rect, outline: Vec<OutlineCurve>) -> Self {
        Self {
            glyph,
            bounds,
            outline,
        }
    }

    /// Glyph info.
    #[inline]
    pub fn glyph(&self) -> &Glyph {
        &self.glyph
    }

    /// Conservative whole number pixel bounding box for this glyph.
    #[inline]
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    /// Draw this glyph outline using a pixel & coverage handling function.
    ///
    /// The callback will be called for each `(x, y)` coordinate inside the pixel bounds
    /// with a coverage value in the range `[0.0, 1.0]`.
    pub fn draw<O: FnMut(u32, u32, f32)>(&self, o: O) {
        use ab_glyph_rasterizer::Rasterizer;
        let offset = self.glyph.position - self.bounds.min;
        let (w, h) = (self.bounds.width() as usize, self.bounds.height() as usize);

        self.outline
            .iter()
            .fold(Rasterizer::new(w, h), |mut rasterizer, curve| match curve {
                OutlineCurve::Line(p0, p1) => {
                    rasterizer.draw_line(*p0 + offset, *p1 + offset);
                    // eprintln!("r.draw_line({:?}, {:?});", *p0 + offset, *p1 + offset);
                    rasterizer
                }
                OutlineCurve::Quad(p0, p1, p2) => {
                    rasterizer.draw_quad(*p0 + offset, *p1 + offset, *p2 + offset);
                    // eprintln!("r.draw_quad({:?}, {:?}, {:?});", *p0 + offset, *p1 + offset, *p2 + offset);
                    rasterizer
                }
                OutlineCurve::Cubic(p0, p1, p2, p3) => {
                    rasterizer.draw_cubic(*p0 + offset, *p1 + offset, *p2 + offset, *p3 + offset);
                    // eprintln!("r.draw_cubic({:?}, {:?}, {:?}, {:?});", *p0 + offset, *p1 + offset, *p2 + offset, *p3 + offset);
                    rasterizer
                }
            })
            .for_each_pixel_2d(o);
    }
}

impl AsRef<Glyph> for OutlinedGlyph {
    #[inline]
    fn as_ref(&self) -> &Glyph {
        self.glyph()
    }
}

/// Glyph outline primitives.
#[derive(Clone, Debug)]
pub enum OutlineCurve {
    /// Straight line from `.0` to `.1`.
    Line(Point, Point),
    /// Quadratic Bézier curve from `.0` to `.2` using `.1` as the control.
    Quad(Point, Point, Point),
    /// Cubic Bézier curve from `.0` to `.3` using `.1` as the control at the beginning of the
    /// curve and `.2` at the end of the curve.
    Cubic(Point, Point, Point, Point),
}

/// A rectangle, with top-left corner at `min`, and bottom-right corner at `max`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}
