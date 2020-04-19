# Unreleased
* `Point` implement `Sub`, `Add`, `SubAssign`, `AddAssign`, `PartialEq`, `PartialOrd` for easier use downstream.
* Switch `Point` `Debug` implementation to output `point(1.2, 3.4)` smaller representation referring to the `point` fn.

# 0.1.1
* Add explicit compile error when building no_std without the "libm" feature.

# 0.1
* Implement zero dependency coverage rasterization for lines, quadratic & cubic beziers.
