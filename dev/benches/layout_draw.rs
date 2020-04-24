use ab_glyph::*;
use approx::assert_relative_eq;
use criterion::{criterion_group, criterion_main, Criterion};

const EXO2_TTF: &[u8] = include_bytes!("../fonts/Exo2-Light.ttf");
const EXO2_OTF: &[u8] = include_bytes!("../fonts/Exo2-Light.otf");

const SENTENCE: &str =
    "a set of words that is complete in itself, typically containing a subject and predicate, \
     conveying a statement, question, exclamation, or command, and consisting of a main \
     clause and sometimes one or more subordinate clauses.";

fn bench_layout_and_draw(c: &mut Criterion) {
    c.bench_function("layout & draw (exo2-otf)", |b| {
        let font = FontRef::try_from_slice(EXO2_OTF).unwrap();
        let mut glyphs = vec![];
        let mut coverage_sum = 0.0;

        b.iter(|| {
            dev::layout_paragraph(
                font.as_scaled(25.0),
                point(100.0, 25.0),
                600.0,
                SENTENCE,
                &mut glyphs,
            );

            coverage_sum = 0.0;

            glyphs
                .drain(..)
                .filter_map(|g| font.outline(g))
                .for_each(|outlined| outlined.draw(|_, _, c| coverage_sum += c));
        });

        // sanity check that work has been done
        assert_relative_eq!(coverage_sum, 6073.028);
    });

    c.bench_function("layout & draw (exo2-ttf)", |b| {
        let font = FontRef::try_from_slice(EXO2_TTF).unwrap();
        let mut glyphs = vec![];
        let mut coverage_sum = 0.0;

        b.iter(|| {
            dev::layout_paragraph(
                font.as_scaled(25.0),
                point(100.0, 25.0),
                600.0,
                SENTENCE,
                &mut glyphs,
            );

            coverage_sum = 0.0;

            glyphs
                .drain(..)
                .filter_map(|g| font.outline(g))
                .for_each(|outlined| outlined.draw(|_, _, c| coverage_sum += c));
        });

        // sanity check that work has been done
        assert_relative_eq!(coverage_sum, 6069.2656);
    });
}

criterion_group!(
    name = layout_and_draw_benches;
    config = Criterion::default();
    targets = bench_layout_and_draw);

criterion_main!(layout_and_draw_benches);
