//! ab_glyph_rasterizer benchmarks
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

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
                rasterizer.for_each_pixel(|idx, alpha| target[idx] = (alpha * 255.0) as u8);

                // ensure the byte array has changed (and not discarded by optimization?).
                assert!(target.iter().any(|a| *a != 0), "target not written to?");
            });
        });

    }};
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
        assert!(target.iter().any(|a| *a != 0), "target not written to?");
    }};
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
                rasterizer.for_each_pixel(|idx, alpha| target[idx] = (alpha * 255.0) as u8);
                // ensure the byte array has changed (and not discarded by optimization?).
                assert!(target.iter().any(|a| *a != 0), "target not written to?");
            });
        });
    }};
}

fn rasterize_ttf_w(c: &mut Criterion) {
    bench_draw!(c, "rasterize_ttf_w", dev::rasterize_ttf_w, (9, 8));
}

fn rasterize_outline_ttf_w(c: &mut Criterion) {
    bench_draw_outline!(c, "rasterize_outline_ttf_w", dev::rasterize_ttf_w, (9, 8));
}

fn accumulate_ttf_w(c: &mut Criterion) {
    bench_accumulate!(c, "accumulate_ttf_w", dev::rasterize_ttf_w, (9, 8));
}

fn rasterize_ttf_tailed_e(c: &mut Criterion) {
    bench_draw!(
        c,
        "rasterize_ttf_tailed_e",
        dev::rasterize_ttf_tailed_e,
        (106, 177)
    );
}

fn rasterize_otf_tailed_e(c: &mut Criterion) {
    bench_draw!(
        c,
        "rasterize_otf_tailed_e",
        dev::rasterize_otf_tailed_e,
        (106, 183)
    );
}

fn rasterize_ttf_biohazard(c: &mut Criterion) {
    bench_draw!(
        c,
        "rasterize_ttf_biohazard",
        dev::rasterize_ttf_biohazard,
        (294, 269)
    );
}

fn rasterize_outline_ttf_biohazard(c: &mut Criterion) {
    bench_draw_outline!(
        c,
        "rasterize_outline_ttf_biohazard",
        dev::rasterize_ttf_biohazard,
        (294, 269)
    );
}

fn accumulate_ttf_biohazard(c: &mut Criterion) {
    bench_accumulate!(
        c,
        "accumulate_ttf_biohazard",
        dev::rasterize_ttf_biohazard,
        (294, 269)
    );
}

criterion_group!(
    name = draw_benches;
    config = Criterion::default()
        .sample_size(200)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(3))
        .noise_threshold(0.025);
    targets = rasterize_ttf_w,
        rasterize_outline_ttf_w,
        accumulate_ttf_w,
        rasterize_ttf_tailed_e,
        rasterize_otf_tailed_e,
        rasterize_ttf_biohazard,
        rasterize_outline_ttf_biohazard,
        accumulate_ttf_biohazard,
);

criterion_main!(draw_benches);
