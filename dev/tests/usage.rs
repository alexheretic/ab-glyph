use ab_glyph::{point, Font, FontRef, Point, Rect};

#[test]
fn build_outline() {
    let font = FontRef::try_from_slice(include_bytes!("../fonts/OpenSans-Italic.ttf")).unwrap();
    let glyph_id = font.glyph_id('x');

    let mut outliner = Outliner::default();

    let rect = font.build_outline(glyph_id, &mut outliner).unwrap();
    assert_eq!(
        rect,
        Rect {
            min: point(-74.0, 1096.0),
            max: point(1030.0, 0.0),
        }
    );

    assert_eq!(
        outliner.calls,
        vec![
            "M point(467.0, 434.0)",
            "L point(121.0, 0.0)",
            "L point(-74.0, 0.0)",
            "L point(401.0, 565.0)",
            "L point(162.0, 1096.0)",
            "L point(332.0, 1096.0)",
            "L point(506.0, 684.0)",
            "L point(836.0, 1096.0)",
            "L point(1030.0, 1096.0)",
            "L point(575.0, 557.0)",
            "L point(827.0, 0.0)",
            "L point(659.0, 0.0)",
            "L point(467.0, 434.0)",
            "Z"
        ]
    );
}

#[derive(Debug, Default)]
struct Outliner {
    calls: Vec<String>,
}

impl ab_glyph::OutlineBuilder for Outliner {
    fn move_to(&mut self, p: Point) {
        self.calls.push(format!("M {p:?}"));
    }

    fn line_to(&mut self, p: Point) {
        self.calls.push(format!("L {p:?}"));
    }

    fn quad_to(&mut self, a: Point, b: Point) {
        self.calls.push(format!("Q {a:?} {b:?}"));
    }

    fn curve_to(&mut self, a: Point, b: Point, c: Point) {
        self.calls.push(format!("C {a:?} {b:?} {c:?}"));
    }

    fn close(&mut self) {
        self.calls.push("Z".into());
    }
}
