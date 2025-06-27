/// Basic required float operations.
pub(crate) trait FloatExt {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn sqrt(self) -> Self;
    #[allow(dead_code)] // available in core since rust 1.84
    fn abs(self) -> Self;
}

impl FloatExt for f32 {
    #[inline]
    fn floor(self) -> Self {
        libm::floorf(self)
    }
    #[inline]
    fn ceil(self) -> Self {
        libm::ceilf(self)
    }
    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
    #[inline]
    fn abs(self) -> Self {
        libm::fabsf(self)
    }
}
