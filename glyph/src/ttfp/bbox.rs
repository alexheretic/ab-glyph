use crate::{OutlineCurve, Point};
use core::convert::TryFrom;

#[derive(Debug)]
pub(crate) struct BoundingBox {
    pub xmin: f32,
    pub ymin: f32,
    pub xmax: f32,
    pub ymax: f32,
}

impl Default for BoundingBox {
    #[inline]
    fn default() -> Self {
        Self {
            xmin: f32::MAX,
            ymin: f32::MAX,
            xmax: f32::MIN,
            ymax: f32::MIN,
        }
    }
}

impl BoundingBox {
    #[inline]
    pub(crate) fn is_zero_sized(&self) -> bool {
        self.xmin >= self.xmax || self.ymin >= self.ymax
    }

    #[inline]
    pub(crate) fn covering(mut self, p: &Point) -> Self {
        self.xmin = self.xmin.min(p.x);
        self.ymin = self.ymin.min(p.y);
        self.xmax = self.xmax.max(p.x);
        self.ymax = self.ymax.max(p.y);
        self
    }
}

impl TryFrom<&[OutlineCurve]> for BoundingBox {
    type Error = ();

    /// Simple computation of a bounding box for a bunch of curves by covering start, finish
    /// and control points. This could be more precise by computing tighter bounds for the
    /// quadratic & cubic curves.
    fn try_from(curves: &[OutlineCurve]) -> Result<Self, Self::Error> {
        let bbox = curves.iter().fold(
            BoundingBox::default(),
            |bbox, next_curve| match next_curve {
                OutlineCurve::Line(p1, p2) => bbox.covering(p1).covering(p2),
                OutlineCurve::Quad(p1, p2, p3) => bbox.covering(p1).covering(p2).covering(p3),
                OutlineCurve::Cubic(p1, p2, p3, p4) => {
                    bbox.covering(p1).covering(p2).covering(p3).covering(p4)
                }
            },
        );

        if bbox.is_zero_sized() {
            Err(())
        } else {
            Ok(bbox)
        }
    }
}

#[test]
fn default_is_zero() {
    assert!(BoundingBox::default().is_zero_sized());
}
