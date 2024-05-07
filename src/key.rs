use core::fmt::Debug;

pub trait Key: Copy + Debug + Eq {
    const ZERO: Self;

    #[must_use]
    fn as_usize(self) -> usize;
    #[must_use]
    fn inc(self) -> Self;
    #[must_use]
    fn dec(self) -> Self;
}

macro_rules! impl_key {
    ($int:ty) => {
        impl Key for $int {
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
    };
}

impl_key!(u8);
impl_key!(u16);
impl_key!(u32);
impl_key!(u64);
impl_key!(usize);
