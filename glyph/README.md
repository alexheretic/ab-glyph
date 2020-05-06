ab_glyph
[![crates.io](https://img.shields.io/crates/v/ab_glyph.svg)](https://crates.io/crates/ab_glyph)
[![Documentation](https://docs.rs/ab_glyph/badge.svg)](https://docs.rs/ab_glyph)
========
Fast API for loading, scaling, positioning and rasterizing OpenType font glyphs.

```rust
use ab_glyph::{FontRef, Font, Glyph, point};

let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;

// Get a glyph for 'q' with a scale & position.
let q_glyph: Glyph = font.glyph_id('q').with_scale_and_position(24.0, point(100.0, 0.0));

// Draw it.
if let Some(q) = font.outline_glyph(q_glyph) {
    q.draw(|x, y, c| { /* draw pixel `(x, y)` with coverage: `c` */ });
}
```

## no_std
no_std environments are supported using `alloc` & [`libm`](https://github.com/rust-lang/libm).
```toml
ab_glyph = { default-features = false, features = ["libm-math"] }
```
