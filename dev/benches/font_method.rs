use std::time::Duration;

use ab_glyph::*;
use approx::assert_relative_eq;
use criterion::{criterion_group, criterion_main, Criterion};

const OPENS_SANS_ITALIC: &[u8] = include_bytes!("../fonts/OpenSans-Italic.ttf");

fn bench_font_glyph_id(c: &mut Criterion) {
    let font = FontRef::try_from_slice(OPENS_SANS_ITALIC).unwrap();

    #[allow(deprecated)]
    c.bench_function("method:Font::glyph_id", |b| {
        let mut glyph = GlyphId(0);

        b.iter(|| {
            glyph = font.glyph_id('x');
        });

        assert_eq!(glyph, GlyphId(91));
    });

    c.bench_function("method:GlyphIdentifier::glyph_id", |b| {
        let c2g = font.glyph_identifier();
        let mut glyph = GlyphId(0);

        b.iter(|| glyph = c2g.glyph_id('x'));

        assert_eq!(glyph, GlyphId(91));
    });

    c.bench_function("method:Font::h_advance", |b| {
        let glyph = GlyphId(91);
        let mut h_advance = 0.0;

        b.iter(|| h_advance = font.h_advance_unscaled(glyph));

        assert_relative_eq!(h_advance, 979.0);
    });

    #[allow(deprecated)]
    c.bench_function("method:Font::kern_unscaled", |b| {
        let glyph = GlyphId(91);
        let glyph2 = GlyphId(92);
        let mut kern = 0.0;

        b.iter(|| kern = font.kern_unscaled(glyph, glyph2));

        assert_relative_eq!(kern, 0.0);
    });

    c.bench_function("method:Kerner::kern_unscaled", |b| {
        let glyph = GlyphId(91);
        let glyph2 = GlyphId(92);
        let kerner = font.kerner();
        let mut kern = 0.0;

        b.iter(|| kern = kerner.kern_unscaled(glyph, glyph2));

        assert_relative_eq!(kern, 0.0);
    });
}

criterion_group!(
    name = font_method_benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(200))
        .sample_size(500)
        .measurement_time(Duration::from_secs(1));
    targets = bench_font_glyph_id,
);

criterion_main!(font_method_benches);
