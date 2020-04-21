//! Render example where each glyph pixel is output as an ascii character.
use ab_glyph::{point, Font, FontRef, FontVec, PxScale, ScaleFont};
use std::io::Write;

const TEXT: &str = "ab_glyph";

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
        draw_ascii(font);
    } else {
        eprintln!("No font specified ... using Exo2-Light.otf");
        let font = FontRef::try_from_slice(include_bytes!("../fonts/Exo2-Light.otf")).unwrap();
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

    // Rasterise to a f32 alpha vec
    let mut pixel_data = vec![0.0; px_width * px_height];
    for g in glyphs {
        if let Some(og) = scaled_font.outline(g) {
            let bounds = og.bounds();
            og.draw(|x, y, v| {
                let x = x as f32 + bounds.min.x;
                let y = y as f32 + bounds.min.y;
                // There's still a possibility that the glyph clips the boundaries of the bitmap
                if x >= 0.0 && (x as usize) < px_width && y >= 0.0 && (y as usize) < px_height {
                    // save the coverage alpha
                    pixel_data[(x as usize + y as usize * px_width)] += v;
                }
            });
        }
    }

    let mapping = b"@#x+=:-. "; // The approximation of greyscale
    let mapping_scale = (mapping.len() - 1) as f32;

    // map the alpha values to a ascii character & print
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\n").unwrap();
    pixel_data
        .into_iter()
        .map(|alpha| ((1.0 - alpha) * mapping_scale + 0.5) as usize)
        .map(|index| mapping[index.max(0).min(mapping.len() - 1)])
        .collect::<Vec<_>>()
        .chunks_exact(px_width)
        .skip_while(|row| row.iter().all(|c| *c == b' '))
        .for_each(|row| {
            handle.write_all(row).unwrap();
            handle.write_all(b"\n").unwrap();
        });
    handle.write_all(b"\n").unwrap();
}
