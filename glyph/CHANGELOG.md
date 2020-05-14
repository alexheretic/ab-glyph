# 0.2
* Add `_unscaled` suffix to  `Font` trait methods that deal with unscaled metrics.
  This helps distinguish `ScaleFont`'s scaled metrics and can avoid unintended behaviour.
* Rename "libm-math" -> "libm" for consistency with ab_glyph_rasterizer. 

# 0.1
* Implement fast glyph layout, outline & drawing primitives.
