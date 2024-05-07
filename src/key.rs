pub trait Key: Copy + Eq {
    const ZERO: Self;

    #[must_use]
    fn as_usize(self) -> usize;
    #[must_use]
    fn inc(self) -> Self;
    #[must_use]
    fn dec(self) -> Self;
}

impl Key for u32 {
    const ZERO: Self = 0;

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    fn inc(self) -> Self {
        self + 1
    }

    #[inline]
    fn dec(self) -> Self {
        self - 1
    }
}
