use ab_glyph::*;
use approx::assert_relative_eq;

/// https://github.com/alexheretic/ab-glyph/issues/29
#[test]
fn ttf_zero_bounding_box_29() {
    let airstrip = FontRef::try_from_slice(include_bytes!("../fonts/airstrip.ttf")).unwrap();

    let outline = airstrip.outline(GlyphId(89)).expect("No outline for glyph");

    assert_relative_eq!(outline.bounds.min.x, 0.0);
    assert_relative_eq!(outline.bounds.max.x, 1102.0);

    assert_relative_eq!(outline.bounds.min.y, 1301.0);
    assert_relative_eq!(outline.bounds.max.y, -3.0);
}
