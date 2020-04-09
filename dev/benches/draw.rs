use criterion::{criterion_group, criterion_main, Criterion};

/// Runs a single rasterization function benchmark.
macro_rules! bench_draw {
    ($criterion:expr, $bench_name:expr, $raster_fn:expr, $const_dimensions:expr) => {{
        // use stack byte array
        let mut target = [0u8; $const_dimensions.0 * $const_dimensions.1];

        // check we got the const dimensions right
        let (w, h) = $raster_fn().dimensions();
        assert_eq!((w, h), $const_dimensions);

        $criterion.bench_function($bench_name, |b| {
            b.iter(|| {
                // call outline functions
                let rasterizer = $raster_fn();
                // draw into the byte array
                rasterizer.for_each_pixel(|idx, alpha| target[idx] = (alpha * 255.0) as u8)
            });
        });
        // ensure the byte array has changed (and not discarded by optimization?).
        assert_ne!(&target as &[u8], &[0u8; $const_dimensions.0 * $const_dimensions.1] as &[u8]);
    }}
}

fn draw_ttf_w(c: &mut Criterion) {
    bench_draw!(c, "draw_ttf_w", dev::rasterize_ttf_w, (9, 8));
}

fn draw_ttf_tailed_e(c: &mut Criterion) {
    bench_draw!(c, "draw_ttf_tailed_e", dev::rasterize_ttf_tailed_e, (98, 158));
}

fn draw_otf_tailed_e(c: &mut Criterion) {
    bench_draw!(c, "draw_otf_tailed_e", dev::rasterize_otf_tailed_e, (106, 183));
}

fn draw_ttf_biohazard(c: &mut Criterion) {
    bench_draw!(c, "draw_ttf_biohazard", dev::rasterize_ttf_biohazard, (294, 269));
}

criterion_group!(
    benches,
    draw_ttf_w,
    draw_ttf_tailed_e,
    draw_otf_tailed_e,
    draw_ttf_biohazard,
);

criterion_main!(benches);
