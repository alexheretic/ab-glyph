use ab_glyph_rasterizer::Rasterizer;
use image::{DynamicImage, LumaA};
use std::io::Cursor;

macro_rules! compare_image {
    ($new_img:expr, $reference_bytes:expr) => {{
        let new_image = $new_img;

        let reference = image::load(
            Cursor::new($reference_bytes as &[u8]),
            image::ImageFormat::Png,
        )
        .expect("!image::load")
        .to_luma_alpha();

        assert_eq!(reference.dimensions(), new_image.dimensions());

        for y in 0..reference.height() {
            for x in 0..reference.width() {
                assert_eq!(
                    reference.get_pixel(x, y),
                    new_image.get_pixel(x, y),
                    "unexpected alpha difference at ({}, {})",
                    x,
                    y
                );
            }
        }
    }};
}

#[test]
fn reference_draw_ttf_w() {
    let new_image = draw_grey_image(dev::rasterize_ttf_w());
    new_image.save("../target/new_ttf_w.png").unwrap();
    compare_image!(new_image, include_bytes!("reference_ttf_w.png"));
}

#[test]
fn reference_draw_ttf_iota() {
    let new_image = draw_grey_image(dev::rasterize_ttf_iota());
    new_image.save("../target/new_ttf_iota.png").unwrap();
    compare_image!(new_image, include_bytes!("reference_ttf_iota.png"));
}

#[test]
fn reference_draw_ttf_biohazard() {
    let new_image = draw_grey_image(dev::rasterize_ttf_biohazard());
    new_image.save("../target/new_ttf_biohazard.png").unwrap();
    compare_image!(new_image, include_bytes!("reference_ttf_biohazard.png"));
}

#[test]
fn reference_draw_otf_tailed_e() {
    let new_image = draw_grey_image(dev::rasterize_otf_tailed_e());
    new_image.save("../target/new_otf_tailed_e.png").unwrap();
    compare_image!(new_image, include_bytes!("reference_otf_tailed_e.png"));
}

#[test]
fn reference_draw_ttf_tailed_e() {
    let new_image = draw_grey_image(dev::rasterize_ttf_tailed_e());
    new_image.save("../target/new_ttf_tailed_e.png").unwrap();
    compare_image!(new_image, include_bytes!("reference_ttf_tailed_e.png"));
}

fn draw_grey_image(rasterizer: Rasterizer) -> image::GrayAlphaImage {
    let (w, h) = rasterizer.dimensions();
    let mut glyph_image = DynamicImage::new_luma_a8(w as _, h as _).to_luma_alpha();

    rasterizer.for_each_pixel_2d(|x, y, alpha| {
        glyph_image.put_pixel(x, y, LumaA([128, (alpha * 255.0).round() as u8]))
    });

    glyph_image
}
