/// An (x, y) coordinate.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl core::fmt::Debug for Point {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "point({:?}, {:?})", self.x, self.y)
    }
}

impl Point {
    #[inline]
    pub(crate) fn distance_to(self, other: Point) -> f32 {
        let d = other - self;
        (d.x * d.x + d.y * d.y).sqrt()
    }
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

impl core::ops::Sub for Point {
    type Output = Point;
    /// Subtract rhs.x from x, rhs.y from y.
    #[inline]
    fn sub(self, rhs: Point) -> Point {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl core::ops::Add for Point {
    type Output = Point;
    /// Add rhs.x to x, rhs.y to y.
    #[inline]
    fn add(self, rhs: Point) -> Point {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl core::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl core::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
