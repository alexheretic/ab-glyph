use ab_glyph_rasterizer::Point;

use crate::{outliner::OutlineCurveBuilder, Rect};

#[derive(Default)]
pub(crate) struct SkrifaCurveBuilder {
    pub(crate) curve_builder: OutlineCurveBuilder,
    pub(crate) bounds: Rect,
}

impl SkrifaCurveBuilder {
    pub fn new() -> Self {
        Self {
            curve_builder: Default::default(),
            bounds: Rect { min: Point { x: core::f32::INFINITY, y: core::f32::NEG_INFINITY }, max: Point { x: core::f32::NEG_INFINITY, y: core::f32::INFINITY } }
        }
    }

    #[inline]
    fn expand_bounds(&mut self, x: f32, y: f32) {
        self.bounds.min.x = self.bounds.min.x.min(x);
        self.bounds.min.y = self.bounds.min.y.max(y);
        self.bounds.max.x = self.bounds.max.x.max(x);
        self.bounds.max.y = self.bounds.max.y.min(y);
    }
}

impl skrifa::outline::OutlinePen for SkrifaCurveBuilder {
    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        self.curve_builder.move_to(x, y);
        self.expand_bounds(x, y);
    }

    #[inline]
    fn line_to(&mut self, x1: f32, y1: f32) {
        self.curve_builder.line_to(x1, y1);
        self.expand_bounds(x1, y1);
    }

    #[inline]
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.curve_builder.quad_to(x1, y1, x2, y2);
        self.expand_bounds(x1, y1);
        self.expand_bounds(x2, y2);
    }

    #[inline]
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.curve_builder.curve_to(x1, y1, x2, y2, x3, y3);
        self.expand_bounds(x1, y1);
        self.expand_bounds(x2, y2);
        self.expand_bounds(x3, y3);
    }

    #[inline]
    fn close(&mut self) {
        self.curve_builder.close();
    }
}
