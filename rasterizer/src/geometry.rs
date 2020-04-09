#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[inline]
pub fn point(x: f32, y: f32) -> Point {
    Point { x, y }
}

#[inline]
pub(crate) fn lerp(t: f32, p0: Point, p1: Point) -> Point {
    point(p0.x + t * (p1.x - p0.x), p0.y + t * (p1.y - p0.y))
}
