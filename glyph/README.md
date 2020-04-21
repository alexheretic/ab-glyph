ab_glyph
[![crates.io](https://img.shields.io/crates/v/ab_glyph.svg)](https://crates.io/crates/ab_glyph)
[![Documentation](https://docs.rs/ab_glyph/badge.svg)](https://docs.rs/ab_glyph)
========
API for loading, scaling, positioning and rasterizing OpenType fonts.

## no_std
no_std environments are supported using `alloc` & [`libm`](https://github.com/rust-lang/libm).
```toml
ab_glyph = { default-features = false, features = ["libm-math"] }
```
