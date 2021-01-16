#[cfg(all(feature = "libm", not(feature = "std")))]
use crate::nostd_float::FloatExt;
use crate::{point, Glyph, Point, PxScaleFactor};
use ab_glyph_rasterizer::Rasterizer;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// A "raw" collection of outline curves for a glyph, unscaled & unpositioned.
#[derive(Clone, Debug)]
pub struct Outline {
    /// Unscaled bounding box.
    pub bounds: Rect,
    /// Unscaled & unpositioned outline curves.
    pub curves: Vec<OutlineCurve>,
}

impl Outline {
    /// Convert unscaled bounds into pixel bounds at a given scale & position.
    pub fn px_bounds(&self, scale_factor: PxScaleFactor, position: Point) -> Rect {
        let Rect { min, max } = self.bounds;

        // Use subpixel fraction in floor/ceil rounding to elimate rounding error
        // from identical subpixel positions
        let (x_trunc, x_fract) = (position.x.trunc(), position.x.fract());
        let (y_trunc, y_fract) = (position.y.trunc(), position.y.fract());

        Rect {
            min: point(
                (min.x * scale_factor.horizontal + x_fract).floor() + x_trunc,
                (min.y * -scale_factor.vertical + y_fract).floor() + y_trunc,
            ),
            max: point(
                (max.x * scale_factor.horizontal + x_fract).ceil() + x_trunc,
                (max.y * -scale_factor.vertical + y_fract).ceil() + y_trunc,
            ),
        }
    }
}

/// A glyph that has been outlined at a scale & position.
#[derive(Clone, Debug)]
pub struct OutlinedGlyph {
    glyph: Glyph,
    // Pixel scale bounds.
    px_bounds: Rect,
    // Scale factor
    scale_factor: PxScaleFactor,
    // Raw outline
    outline: Outline,
}

impl OutlinedGlyph {
    /// Constructs an `OutlinedGlyph` from the source `Glyph`, pixel bounds
    /// & relatively positioned outline curves.
    #[inline]
    pub fn new(glyph: Glyph, outline: Outline, scale_factor: PxScaleFactor) -> Self {
        // work this out now as it'll usually be used more than once
        let px_bounds = outline.px_bounds(scale_factor, glyph.position);

        Self {
            glyph,
            px_bounds,
            scale_factor,
            outline,
        }
    }

    /// Glyph info.
    #[inline]
    pub fn glyph(&self) -> &Glyph {
        &self.glyph
    }

    #[deprecated = "Renamed to `px_bounds`"]
    #[doc(hidden)]
    pub fn bounds(&self) -> Rect {
        self.px_bounds()
    }

    /// Conservative whole number pixel bounding box for this glyph.
    #[inline]
    pub fn px_bounds(&self) -> Rect {
        self.px_bounds
    }

    /// Draw this glyph outline using a pixel & coverage handling function.
    ///
    /// The callback will be called for each `(x, y)` pixel coordinate inside the bounds
    /// with a coverage value in the range `[0.0, 1.0]` indicating how much the glyph covered
    /// that pixel.
    #[inline]
    pub fn draw<O>(&self, o: O)
    where
        O: FnMut(u32, u32, f32),
    {
        self.inner_draw_using(
            &mut Rasterizer::new(self.px_bounds.width() as _, self.px_bounds.height() as _),
            o,
        )
    }

    /// Draw this glyph outline using a pixel & coverage handling function.
    ///
    /// Similar to [`OutlinedGlyph::draw`] but takes a `&mut Rasterizer` which can minimise/eliminate
    /// allocation by allowing reuse.
    #[inline]
    pub fn draw_using<O>(&self, rasterizer: &mut Rasterizer, o: O)
    where
        O: FnMut(u32, u32, f32),
    {
        rasterizer.reset(self.px_bounds.width() as _, self.px_bounds.height() as _);
        self.inner_draw_using(rasterizer, o)
    }

    // Note: Must be called with a correctly sized & empty `rasterizer`.
    fn inner_draw_using<O>(&self, rasterizer: &mut Rasterizer, o: O)
    where
        O: FnMut(u32, u32, f32),
    {
        let h_factor = self.scale_factor.horizontal;
        let v_factor = -self.scale_factor.vertical;
        let offset = self.glyph.position - self.px_bounds.min;
        let scale_up = |&Point { x, y }| point(x * h_factor, y * v_factor);

        self.outline
            .curves
            .iter()
            .fold(rasterizer, |rasterizer, curve| match curve {
                OutlineCurve::Line(p0, p1) => {
                    // eprintln!("r.draw_line({:?}, {:?});",
                    //     scale_up(p0) + offset, scale_up(p1) + offset);
                    rasterizer.draw_line(scale_up(p0) + offset, scale_up(p1) + offset);
                    rasterizer
                }
                OutlineCurve::Quad(p0, p1, p2) => {
                    // eprintln!("r.draw_quad({:?}, {:?}, {:?});",
                    //     scale_up(p0) + offset, scale_up(p1) + offset, scale_up(p2) + offset);
                    rasterizer.draw_quad(
                        scale_up(p0) + offset,
                        scale_up(p1) + offset,
                        scale_up(p2) + offset,
                    );
                    rasterizer
                }
                OutlineCurve::Cubic(p0, p1, p2, p3) => {
                    // eprintln!("r.draw_cubic({:?}, {:?}, {:?}, {:?});",
                    //     scale_up(p0) + offset, scale_up(p1) + offset, scale_up(p2) + offset, scale_up(p3) + offset);
                    rasterizer.draw_cubic(
                        scale_up(p0) + offset,
                        scale_up(p1) + offset,
                        scale_up(p2) + offset,
                        scale_up(p3) + offset,
                    );
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
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
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
