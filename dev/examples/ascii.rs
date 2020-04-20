//! Render example where each glyph pixel is output as an ascii character.
use ab_glyph::{point, ttf_parser, Font, PxScale, ScaleFont};
use std::io::Write;

const TEXT: &str = "ab_glyph";

fn main() {
    if let Some(font_path) = std::env::args().nth(1) {
        let font_path = std::env::current_dir().unwrap().join(font_path);
        let data = std::fs::read(&font_path).unwrap();
        let font = ttf_parser::OwnedFont::try_from_vec(data, 0).unwrap_or_else(|| {
            panic!(format!(
                "error constructing a Font from data at {:?}",
                font_path
            ));
        });
        draw_ascii(font);
    } else {
        eprintln!("No font specified ... using Exo2-Light.otf");
        let font =
            ttf_parser::Font::from_data(include_bytes!("../fonts/Exo2-Light.otf"), 0).unwrap();
        draw_ascii(font);
    };
}

fn draw_ascii<F: Font>(font: F) {
    // Desired font pixel height
    let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
    let px_height = height.ceil() as usize;

    // 2x h-scale works better in ascii
    let scale = PxScale {
        x: height * 2.0,
        y: height,
    };

    let scaled_font = font.into_scaled(scale);

    let mut glyphs = Vec::new();
    dev::layout_paragraph(&scaled_font, point(0.0, 0.0), 9999.0, TEXT, &mut glyphs);

    let px_width = glyphs
        .iter()
        .last()
        .map(|g| g.position.x + scaled_font.h_advance(g.id))
        .unwrap_or(0.0)
        .ceil() as usize;

    println!("width: {}, height: {}", px_width, px_height);

    // Rasterise directly into ASCII art.
    let mut pixel_data = vec![b' '; px_width * px_height];
    let mapping = b"@#x+=:-. "; // The approximation of greyscale
    let mapping_scale = (mapping.len() - 1) as f32;
    for g in glyphs {
        if let Some(og) = scaled_font.outline(g) {
            let bounds = og.bounds();
            og.draw(|x, y, v| {
                // v should be in the range 0.0 to 1.0
                let i = ((1.0 - v) * mapping_scale + 0.5) as usize;
                // so something's wrong if you get $ in the output.
                let c = mapping.get(i).copied().unwrap_or(b' ');
                let x = x as f32 + bounds.min.x;
                let y = y as f32 + bounds.min.y;
                // There's still a possibility that the glyph clips the boundaries of the bitmap
                if x >= 0.0 && (x as usize) < px_width && y >= 0.0 && (y as usize) < px_height {
                    pixel_data[(x as usize + y as usize * px_width)] = c;
                }
            });
        }
    }

    // Print it out
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\n").unwrap();
    (0..px_height)
        .map(|j| &pixel_data[j * px_width..(j + 1) * px_width])
        .skip_while(|row| row.iter().all(|p| *p == b' '))
        .for_each(|row| {
            handle.write_all(row).unwrap();
            handle.write_all(b"\n").unwrap();
        });
    handle.write_all(b"\n").unwrap();
}
