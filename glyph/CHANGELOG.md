# 0.2.4
* Update _ttf-parser_ to `0.7` adding CID font support.

# 0.2.3
* Add `v_advance` & `v_side_bearing` methods to `ScaleFont` + `_unscaled` variants to `Font`.

# 0.2.2
* Add `Font::glyph_bounds` method, similar to glyph_brush's `glyph_bounds` but for a single glyph.
* Rename `OutlinedGlyph::bounds` to `OutlinedGlyph::px_bounds` for clarity.

# 0.2.1
* Update _ttf-parser_ to `0.6`.

# 0.2
* Add `_unscaled` suffix to  `Font` trait methods that deal with unscaled metrics.
  This helps distinguish `ScaleFont`'s scaled metrics and can avoid unintended behaviour.
* Rename "libm-math" -> "libm" for consistency with _ab_glyph_rasterizer_. 

# 0.1
* Implement fast glyph layout, outline & drawing primitives.
