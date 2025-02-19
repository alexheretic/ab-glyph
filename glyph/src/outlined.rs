#[cfg(all(feature = "libm", not(feature = "std")))]
use crate::nostd_float::FloatExt;
use crate::{point, Glyph, Point, PxScaleFactor};
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
    ///
    /// See [`OutlinedGlyph::px_bounds`].
    pub fn px_bounds(&self, scale_factor: PxScaleFactor, position: Point) -> Rect {
        let Rect { min, max } = self.bounds;

        // Use subpixel fraction in floor/ceil rounding to eliminate rounding error
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

    /// Conservative whole number pixel bounding box for this glyph outline.
    /// The returned rect is exactly large enough to [`Self::draw`] into.
    ///
    /// The rect holds bounding coordinates in the same coordinate space as the [`Glyph::position`].
    ///
    /// Note: These bounds depend on the glyph outline. That outline is *not* necessarily bound
    ///       by the layout/`glyph_bounds()` bounds.
    /// * The min.x bound may be greater or smaller than the [`Glyph::position`] x.
    ///   E.g. if a glyph at position x=0 has an outline going off to the left a bit, min.x will be negative.
    /// * The max.x bound may be greater/smaller than the `position.x + h_advance`.
    /// * The min.y bound may be greater/smaller than the `position.y - ascent`.
    /// * The max.y bound may be greater/smaller than the `position.y - descent`.
    ///
    /// Pixel bounds coordinates should not be used for layout logic.
    #[inline]
    pub fn px_bounds(&self) -> Rect {
        self.px_bounds
    }

    /// Draw this glyph outline using a pixel & coverage handling function.
    ///
    /// The callback will be called for each `(x, y)` pixel coordinate inside the bounds
    /// with a coverage value indicating how much the glyph covered that pixel.
    ///
    /// A coverage value of `0.0` means the pixel is totally uncovered by the glyph.
    /// A value of `1.0` or greater means fully covered.
    pub fn draw<O: FnMut(u32, u32, f32)>(&self, o: O) {
        use ab_glyph_rasterizer::Rasterizer;
        let h_factor = self.scale_factor.horizontal;
        let v_factor = -self.scale_factor.vertical;
        let offset = self.glyph.position - self.px_bounds.min;
        let (w, h) = (
            self.px_bounds.width() as usize,
            self.px_bounds.height() as usize,
        );

        let scale_up = |&Point { x, y }| point(x * h_factor, y * v_factor);

        self.outline
            .curves
            .iter()
            .fold(Rasterizer::new(w, h), |mut rasterizer, curve| match curve {
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
