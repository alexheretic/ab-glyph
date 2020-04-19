use crate::{glyph::*, outlined::*};

pub(crate) struct OutlineCurveBuilder {
    h_px_factor: f32,
    v_px_factor: f32,
    last: Point,
    last_move: Option<Point>,
    outline: Vec<OutlineCurve>,
}

impl OutlineCurveBuilder {
    #[inline]
    pub(crate) fn new(h_px_factor: f32, v_px_factor: f32) -> Self {
        Self {
            h_px_factor,
            v_px_factor,
            last: <_>::default(),
            last_move: None,
            outline: <_>::default(),
        }
    }

    #[inline]
    pub(crate) fn take_outline(self) -> Vec<OutlineCurve> {
        self.outline
    }
}

impl ttf_parser::OutlineBuilder for OutlineCurveBuilder {
    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        self.last = Point {
            x: x as f32 * self.h_px_factor,
            y: -y as f32 * self.v_px_factor,
        };
        self.last_move = Some(self.last);
    }

    #[inline]
    fn line_to(&mut self, x1: f32, y1: f32) {
        let p1 = Point {
            x: x1 as f32 * self.h_px_factor,
            y: -y1 as f32 * self.v_px_factor,
        };

        self.outline.push(OutlineCurve::Line(self.last, p1));
        self.last = p1;
    }

    #[inline]
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let p1 = Point {
            x: x1 as f32 * self.h_px_factor,
            y: -y1 as f32 * self.v_px_factor,
        };
        let p2 = Point {
            x: x2 as f32 * self.h_px_factor,
            y: -y2 as f32 * self.v_px_factor,
        };

        self.outline.push(OutlineCurve::Quad(self.last, p1, p2));
        self.last = p2;
    }

    #[inline]
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let p1 = Point {
            x: x1 as f32 * self.h_px_factor,
            y: -y1 as f32 * self.v_px_factor,
        };
        let p2 = Point {
            x: x2 as f32 * self.h_px_factor,
            y: -y2 as f32 * self.v_px_factor,
        };
        let p3 = Point {
            x: x3 as f32 * self.h_px_factor,
            y: -y3 as f32 * self.v_px_factor,
        };

        self.outline
            .push(OutlineCurve::Cubic(self.last, p1, p2, p3));
        self.last = p3;
    }

    #[inline]
    fn close(&mut self) {
        if let Some(m) = self.last_move {
            self.outline.push(OutlineCurve::Line(self.last, m));
        }
    }
}
