use ab_glyph::{point, Font, FontRef, FontVec, PxScale, ScaleFont};
use image::{DynamicImage, Rgba};

const TEXT: &str = "This is ab_glyph rendered into a png!";

fn main() {
    if let Some(font_path) = std::env::args().nth(1) {
        let font_path = std::env::current_dir().unwrap().join(font_path);
        let data = std::fs::read(&font_path).unwrap();
        let font = FontVec::try_from_vec(data).unwrap_or_else(|_| {
            panic!(format!(
                "error constructing a Font from data at {:?}",
                font_path
            ));
        });
        draw_image(font);
    } else {
        eprintln!("No font specified ... using OpenSans-Italic.ttf");
        let font = FontRef::try_from_slice(include_bytes!("../fonts/OpenSans-Italic.ttf")).unwrap();
        draw_image(font);
    };
}

fn draw_image<F: Font>(font: F) {
    // The font size to use
    let scale = PxScale::from(45.0);

    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    dev::layout_paragraph(scaled_font, point(20.0, 20.0), 9999.0, TEXT, &mut glyphs);

    // Use a dark red colour
    let colour = (150, 0, 0);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba();

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    colour.0,
                    colour.1,
                    colour.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    // Save the image to a png file
    image.save("image_example.png").unwrap();
    println!("Generated: image_example.png");
}
