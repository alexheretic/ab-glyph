// Forked/repurposed from `font-rs` code: https://github.com/raphlinus/font-rs
// Cubic bezier drawing adapted from stb_truetype: https://github.com/nothings/stb
#[cfg(all(feature = "libm", not(feature = "std")))]
use crate::nostd_float::FloatExt;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::geometry::*;

/// Coverage rasterizer for lines, quadratic & cubic beziers.
pub struct Rasterizer {
    width: usize,
    height: usize,
    a: Vec<f32>,
}

impl Rasterizer {
    /// Allocates a new rasterizer that can draw onto a `width` x `height` alpha grid.
    ///
    /// ```
    /// use ab_glyph_rasterizer::Rasterizer;
    /// let mut rasterizer = Rasterizer::new(14, 38);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            a: vec![0.0; width * height + 4],
        }
    }

    /// Returns the dimensions the rasterizer was built to draw to.
    ///
    /// ```
    /// # use ab_glyph_rasterizer::*;
    /// let rasterizer = Rasterizer::new(9, 8);
    /// assert_eq!((9, 8), rasterizer.dimensions());
    /// ```
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Adds a straight line from `p0` to `p1` to the outline.
    ///
    /// ```
    /// # use ab_glyph_rasterizer::*;
    /// # let mut rasterizer = Rasterizer::new(9, 8);
    /// rasterizer.draw_line(point(0.0, 0.48), point(1.22, 0.48));
    /// ```
    pub fn draw_line(&mut self, p0: Point, p1: Point) {
        if (p0.y - p1.y).abs() < core::f32::EPSILON {
            return;
        }
        let (dir, p0, p1) = if p0.y < p1.y {
            (1.0, p0, p1)
        } else {
            (-1.0, p1, p0)
        };
        let dxdy = (p1.x - p0.x) / (p1.y - p0.y);
        let mut x = p0.x;
        let y0 = p0.y as usize; // note: implicit max of 0 because usize (TODO: really true?)
        if p0.y < 0.0 {
            x -= p0.y * dxdy;
        }
        for y in y0..self.height.min(p1.y.ceil() as usize) {
            let linestart = y * self.width;
            let dy = ((y + 1) as f32).min(p1.y) - (y as f32).max(p0.y);
            let xnext = x + dxdy * dy;
            let d = dy * dir;
            let (x0, x1) = if x < xnext { (x, xnext) } else { (xnext, x) };
            let x0floor = x0.floor();
            let x0i = x0floor as i32;
            let x1ceil = x1.ceil();
            let x1i = x1ceil as i32;
            if x1i <= x0i + 1 {
                let xmf = 0.5 * (x + xnext) - x0floor;
                self.a[linestart.wrapping_add(x0i as usize)] += d - d * xmf;
                self.a[linestart + (x0i + 1) as usize] += d * xmf;
            } else {
                let s = (x1 - x0).recip();
                let x0f = x0 - x0floor;
                let a0 = 0.5 * s * (1.0 - x0f) * (1.0 - x0f);
                let x1f = x1 - x1ceil + 1.0;
                let am = 0.5 * s * x1f * x1f;
                self.a[linestart.wrapping_add(x0i as usize)] += d * a0;
                if x1i == x0i + 2 {
                    self.a[linestart + (x0i + 1) as usize] += d * (1.0 - a0 - am);
                } else {
                    let a1 = s * (1.5 - x0f);
                    self.a[linestart + (x0i + 1) as usize] += d * (a1 - a0);
                    for xi in x0i + 2..x1i - 1 {
                        self.a[linestart + xi as usize] += d * s;
                    }
                    let a2 = a1 + (x1i - x0i - 3) as f32 * s;
                    self.a[linestart + (x1i - 1) as usize] += d * (1.0 - a2 - am);
                }
                self.a[linestart + x1i as usize] += d * am;
            }
            x = xnext;
        }
    }

    /// Adds a quadratic Bézier curve from `p0` to `p2` to the outline using `p1` as the control.
    ///
    /// ```
    /// # use ab_glyph_rasterizer::*;
    /// # let mut rasterizer = Rasterizer::new(14, 38);
    /// rasterizer.draw_quad(point(6.2, 34.5), point(7.2, 34.5), point(9.2, 34.0));
    /// ```
    pub fn draw_quad(&mut self, p0: Point, p1: Point, p2: Point) {
        let devx = p0.x - 2.0 * p1.x + p2.x;
        let devy = p0.y - 2.0 * p1.y + p2.y;
        let devsq = devx * devx + devy * devy;
        if devsq < 0.333 {
            self.draw_line(p0, p2);
            return;
        }
        let tol = 3.0;
        let n = 1 + (tol * devsq).sqrt().sqrt().floor() as usize;
        let mut p = p0;
        let nrecip = (n as f32).recip();
        let mut t = 0.0;
        for _i in 0..n - 1 {
            t += nrecip;
            let pn = lerp(t, lerp(t, p0, p1), lerp(t, p1, p2));
            self.draw_line(p, pn);
            p = pn;
        }
        self.draw_line(p, p2);
    }

    /// Adds a cubic Bézier curve from `p0` to `p3` to the outline using `p1` as the control
    /// at the beginning of the curve and `p2` at the end of the curve.
    ///
    /// ```
    /// # use ab_glyph_rasterizer::*;
    /// # let mut rasterizer = Rasterizer::new(12, 20);
    /// rasterizer.draw_cubic(
    ///     point(10.3, 16.4),
    ///     point(8.6, 16.9),
    ///     point(7.7, 16.5),
    ///     point(8.2, 15.2),
    /// );
    /// ```
    pub fn draw_cubic(&mut self, p0: Point, p1: Point, p2: Point, p3: Point) {
        self.tesselate_cubic(p0, p1, p2, p3, 0);
    }

    // stb_truetype style cubic approximation by lines.
    fn tesselate_cubic(&mut self, p0: Point, p1: Point, p2: Point, p3: Point, n: u8) {
        // ...I'm not sure either ¯\_(ツ)_/¯
        const OBJSPACE_FLATNESS: f32 = 0.35;
        const OBJSPACE_FLATNESS_SQUARED: f32 = OBJSPACE_FLATNESS * OBJSPACE_FLATNESS;
        const MAX_RECURSION_DEPTH: u8 = 16;

        let dx0 = p1.x - p0.x;
        let dy0 = p1.y - p0.y;
        let dx1 = p2.x - p1.x;
        let dy1 = p2.y - p1.y;
        let dx2 = p3.x - p2.x;
        let dy2 = p3.y - p2.y;
        let dx = p3.x - p0.x;
        let dy = p3.y - p0.y;
        let longlen = (dx0 * dx0 + dy0 * dy0).sqrt()
            + (dx1 * dx1 + dy1 * dy1).sqrt()
            + (dx2 * dx2 + dy2 * dy2).sqrt();
        let shortlen = (dx * dx + dy * dy).sqrt();
        let flatness_squared = longlen * longlen - shortlen * shortlen;

        if n < MAX_RECURSION_DEPTH && flatness_squared > OBJSPACE_FLATNESS_SQUARED {
            let p01 = lerp(0.5, p0, p1);
            let p12 = lerp(0.5, p1, p2);
            let p23 = lerp(0.5, p2, p3);

            let pa = lerp(0.5, p01, p12);
            let pb = lerp(0.5, p12, p23);

            let mp = lerp(0.5, pa, pb);

            self.tesselate_cubic(p0, p01, pa, mp, n + 1);
            self.tesselate_cubic(mp, pb, p23, p3, n + 1);
        } else {
            self.draw_line(p0, p3);
        }
    }

    /// Run a callback for each pixel index & alpha, with indices in `0..width * height`.
    ///
    /// ```
    /// # use ab_glyph_rasterizer::*;
    /// # let (width, height) = (1, 1);
    /// # let mut rasterizer = Rasterizer::new(width, height);
    /// let mut pixels = vec![0u8; width * height];
    /// rasterizer.for_each_pixel(|index, alpha| {
    ///     pixels[index] = (alpha * 255.0).round() as u8;
    /// });
    /// ```
    pub fn for_each_pixel<O: FnMut(usize, f32)>(&self, mut px_fn: O) {
        let mut acc = 0.0;
        self.a[..self.width * self.height]
            .iter()
            .enumerate()
            .for_each(|(idx, c)| {
                acc += c;
                px_fn(idx, acc.abs().min(1.0));
            });
    }

    /// Run a callback for each pixel x position, y position & alpha.
    ///
    /// Convenience wrapper for `for_each_pixel`.
    ///
    /// ```
    /// # use ab_glyph_rasterizer::*;
    /// # let (width, height) = (1, 1);
    /// # let mut rasterizer = Rasterizer::new(width, height);
    /// # struct Img;
    /// # impl Img { fn set_pixel(&self, x: u32, y: u32, a: u8) {} }
    /// # let image = Img;
    /// rasterizer.for_each_pixel_2d(|x, y, alpha| {
    ///     image.set_pixel(x, y, (alpha * 255.0).round() as u8);
    /// });
    /// ```
    pub fn for_each_pixel_2d<O: FnMut(u32, u32, f32)>(&self, mut px_fn: O) {
        let width32 = self.width as u32;
        self.for_each_pixel(|idx, alpha| px_fn(idx as u32 % width32, idx as u32 / width32, alpha));
    }
}

/// ```
/// let rasterizer = ab_glyph_rasterizer::Rasterizer::new(3, 4);
/// assert_eq!(&format!("{:?}", rasterizer), "Rasterizer { width: 3, height: 4 }");
/// ```
impl core::fmt::Debug for Rasterizer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rasterizer")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
