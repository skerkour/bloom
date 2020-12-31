use crate::traits::SignedInteger;
#[cfg(__standback_before_1_43)]
use crate::v1_43::int_v1_43;

pub trait int_v1_45: SignedInteger {
    fn saturating_neg(self) -> Self;
    fn saturating_abs(self) -> Self;
}

macro_rules! impl_int_v1_45 {
    ($($type:ty),*) => {$(
        impl int_v1_45 for $type {
            fn saturating_neg(self) -> Self {
                if self == Self::MIN {
                    Self::MAX
                } else {
                    -self
                }
            }

            fn saturating_abs(self) -> Self {
                if self.is_negative() {
                    self.saturating_neg()
                } else {
                    self
                }
            }
        }
    )*};
}

impl_int_v1_45![i8, i16, i32, i64, i128, isize];
