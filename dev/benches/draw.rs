use criterion::{criterion_group, criterion_main, Criterion};

/// Runs a single rasterization function benchmark.
/// Calls draw_* outline function then `for_each_pixel` into a byte array.
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

/// Calls draw_* outline functions without timing the `for_each_pixel` (accumulation).
macro_rules! bench_draw_outline {
    ($criterion:expr, $bench_name:expr, $raster_fn:expr, $const_dimensions:expr) => {{
        // use stack byte array
        let mut target = [0u8; $const_dimensions.0 * $const_dimensions.1];

        let mut rasterizer = $raster_fn();

        $criterion.bench_function($bench_name, |b| {
            b.iter(|| {
                // call outline functions
                rasterizer = $raster_fn();
            });
        });

        assert_eq!(rasterizer.dimensions(), $const_dimensions);

        // draw into the byte array
        rasterizer.for_each_pixel(|idx, alpha| target[idx] = (alpha * 255.0) as u8);
        // ensure the byte array has changed (and not discarded by optimization?).
        assert_ne!(&target as &[u8], &[0u8; $const_dimensions.0 * $const_dimensions.1] as &[u8]);
    }}
}

/// Calls `for_each_pixel` on a pre-outlined rasterizer.
macro_rules! bench_accumulate {
    ($criterion:expr, $bench_name:expr, $raster_fn:expr, $const_dimensions:expr) => {{
        // use stack byte array
        let mut target = [0u8; $const_dimensions.0 * $const_dimensions.1];

        // call outline functions
        let rasterizer = $raster_fn();
        // check we got the const dimensions right
        let (w, h) = rasterizer.dimensions();
        assert_eq!((w, h), $const_dimensions);

        $criterion.bench_function($bench_name, |b| {
            b.iter(|| {
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

fn draw_outline_ttf_w(c: &mut Criterion) {
    bench_draw_outline!(c, "draw_outline_ttf_w", dev::rasterize_ttf_w, (9, 8));
}

fn accumulate_ttf_w(c: &mut Criterion) {
    bench_accumulate!(c, "accumulate_ttf_w", dev::rasterize_ttf_w, (9, 8));
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

fn draw_outline_ttf_biohazard(c: &mut Criterion) {
    bench_draw_outline!(c, "draw_outline_ttf_biohazard", dev::rasterize_ttf_biohazard, (294, 269));
}

fn accumulate_ttf_biohazard(c: &mut Criterion) {
    bench_accumulate!(c, "accumulate_ttf_biohazard", dev::rasterize_ttf_biohazard, (294, 269));
}

criterion_group!(
    draw_benches,
    draw_ttf_w,
    draw_outline_ttf_w,
    accumulate_ttf_w,
    draw_ttf_tailed_e,
    draw_otf_tailed_e,
    draw_ttf_biohazard,
    draw_outline_ttf_biohazard,
    accumulate_ttf_biohazard,
);

criterion_main!(draw_benches);
