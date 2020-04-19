use crate::glyph::*;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct OutlinedGlyph {
    glyph: Glyph,
    bounds: Rect,
    outline: Vec<OutlineCurve>,
}

impl OutlinedGlyph {
    #[inline]
    pub fn new(glyph: Glyph, bounds: Rect, outline: Vec<OutlineCurve>) -> Self {
        Self {
            glyph,
            bounds,
            outline,
        }
    }

    #[inline]
    pub fn glyph(&self) -> &Glyph {
        &self.glyph
    }

    #[inline]
    pub fn bounds(&self) -> Rect {
        self.bounds
    }

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

#[derive(Clone, Debug)]
pub enum OutlineCurve {
    Line(Point, Point),
    Quad(Point, Point, Point),
    Cubic(Point, Point, Point, Point),
}

/// A rectangle, with top-left corner at `min`, and bottom-right corner at max`.
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
