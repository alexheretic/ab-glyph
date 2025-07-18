# 0.2.31
* Add "gvar-alloc" feature enabled by default (activates _ttf-parser_ "gvar-alloc" feature).
  Provides full gvar table support.

# 0.2.30
* Add `Font::italic_angle` for accessing the slant angle.

# 0.2.29
* Update _ttf-parser_ to `0.25`.

# 0.2.28
* Update _ttf-parser_ to `0.24`.
* Clarify `OutlinedGlyph::px_bounds`, `Font::glyph_bounds` documentation, 
  describe how they relate to drawing and each other.

# 0.2.27
* Add `Font` glyph layout concept documentation demonstrating "ascent", "descent", "h_side_bearing", 
  "h_advance", "height", "line_gap", "baseline".

# 0.2.26
* Update _ttf-parser_ to `0.21`.

# 0.2.25
* Add `Font::glyph_svg_image` for accessing glyph svg data.

# 0.2.24
* Add `Font::font_data` for accessing underlying raw font data.

# 0.2.23
* Update _ttf-parser_ to `0.20`.

# 0.2.22
* Add `v2::GlyphImage` and `Font::glyph_raster_image2` to expose width and height info.
* Deprecate `Font::glyph_raster_image` & `GlyphImage`.
* Improve `OutlinedGlyph::draw` documentation.

# 0.2.21
* Update _ttf-parser_ to `0.19`.
* Add `GlyphImageFormat` variants `BitmapMono`, `BitmapMonoPacked`, `BitmapGray2`, `BitmapGray2Packed`,
  `BitmapGray4`, `BitmapGray4Packed`, `BitmapGray8`, `BitmapPremulBgra32`.
* `Font::h_advance_unscaled`, `h_side_bearing_unscaled`, `v_advance_unscaled`, `v_side_bearing_unscaled`
  and related `ScaleFont` methods now return `0.0` if the font does not define that value.
  Previously calls would panic when fonts lacked support.
* Use edition 2021.

# 0.2.20
* Add `FontVec::as_slice`, `FontVec::into_vec`.

# 0.2.19
* Update _ttf-parser_ to `0.18`.

# 0.2.18
* Update _ttf-parser_ to `0.17`.

# 0.2.17
* Add `VariableFont` trait implemented by `FontRef` & `FontVec`.
  Provides `variations` & `set_variation` functions.
* Add default enabled feature `variable-fonts`.

# 0.2.16
* Add `Font::pt_to_px_scale` to ease converting point size to `PxScale`.
* Add `PxScale::round`.

# 0.2.15
* Fix some font outlines by always trying to "close" them at the end. Fixes _Cantarell-VF.otf_ outlining.

# 0.2.14
* Update _ttf-parser_ to `0.15`.

# 0.2.13
* Update _ttf-parser_ to `0.14`.

# 0.2.12
* Update _owned-ttf-parser_ to `0.13.2`.
* Pre-parse cmap & kern subtables on all `Font` variants at initialization. This provides
  much faster `glyph_id` & `kern` method performance, results in 25-30% faster layout
  benchmark performance.

# 0.2.11
* `Font::outline` will return `None` for rare invalid/empty glyph bounds instead of panicking.
* Add `Font::glyph_raster_image` for color emoji fonts.

# 0.2.10
* Update _ttf-parser_ to `0.12`.

# 0.2.9
* Update _ttf-parser_ to `0.11`.

# 0.2.8
* Add fallback bounding box calculation for malformed font glyphs with zero sized boxes.
* Update _ttf-parser_ to `0.10`.

# 0.2.7
* Update _ttf-parser_ to `0.9`.

# 0.2.6
* Add `Font::codepoint_ids` method for iterating over `(GlyphId, char)` pairs.
* Clarify documentation.

# 0.2.5
* Add `Font::units_per_em` + documentation on unscaled font units.
* Update _ttf-parser_ to `0.8`.

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
