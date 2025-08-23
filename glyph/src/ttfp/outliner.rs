use crate::outliner::OutlineCurveBuilder;

#[derive(Default)]
pub(crate) struct TtfpCurveBuilder(pub(crate) OutlineCurveBuilder);

impl owned_ttf_parser::OutlineBuilder for TtfpCurveBuilder {
    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y);
    }

    #[inline]
    fn line_to(&mut self, x1: f32, y1: f32) {
        self.0.line_to(x1, y1);
    }

    #[inline]
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.0.quad_to(x1, y1, x2, y2);
    }

    #[inline]
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.0.curve_to(x1, y1, x2, y2, x3, y3);
    }

    #[inline]
    fn close(&mut self) {
        self.0.close();
    }
}
