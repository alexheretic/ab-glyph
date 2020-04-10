/// An (x, y) coordinate.
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// [`Point`](struct.Point.html) constructor.
///
/// ```
/// use ab_glyph_rasterizer::{point, Point};
/// let control: Point = point(0.1, 23.2);
/// ```
#[inline]
pub fn point(x: f32, y: f32) -> Point {
    Point { x, y }
}

/// Linear interpolation between points.
#[inline]
pub(crate) fn lerp(t: f32, p0: Point, p1: Point) -> Point {
    point(p0.x + t * (p1.x - p0.x), p0.y + t * (p1.y - p0.y))
}
