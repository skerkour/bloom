#![allow(deprecated, clippy::missing_const_for_fn)]

use core::ops::{Div, DivAssign, Mul, MulAssign, Neg, Not};
#[cfg(feature = "serde")]
use standback::convert::TryInto;
use Sign::{Negative, Positive, Zero};

/// Contains the sign of a value: positive, negative, or zero.
///
/// For ease of use, `Sign` implements [`Mul`] and [`Div`] on all signed numeric
/// types. `Sign`s can also be multiplied and divided by another `Sign`, which
/// follows the same rules as real numbers.
#[repr(i8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(into = "crate::serde::Sign"))]
#[deprecated(
    since = "0.2.7",
    note = "The only use for this (obtaining the sign of a `Duration`) can be replaced with \
            `Duration::is_{positive|negative|zero}`"
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Sign {
    /// A positive value.
    Positive = 1,

    /// A negative value.
    Negative = -1,

    /// A value that is exactly zero.
    Zero = 0,
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Sign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        crate::serde::Sign::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}
impl Default for Sign {
    /// `Sign` defaults to `Zero`.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::Sign;
    /// assert_eq!(Sign::default(), Sign::Zero);
    /// ```
    fn default() -> Self {
        Zero
    }
}

macro_rules! sign_mul {
    ($($type:ty),+ $(,)?) => {
        $(
            impl Mul<$type> for Sign {
                type Output = $type;

                #[allow(trivial_numeric_casts)]
                fn mul(self, rhs: $type) -> Self::Output {
                    (self as i8) as $type * rhs
                }
            }

            impl Mul<Sign> for $type {
                type Output = Self;

                fn mul(self, rhs: Sign) -> Self::Output {
                    rhs * self
                }
            }

            impl MulAssign<Sign> for $type {
                #[allow(trivial_numeric_casts)]
                fn mul_assign(&mut self, rhs: Sign) {
                    *self *= rhs as i8 as $type;
                }
            }

            impl Div<Sign> for $type {
                type Output = Self;

                fn div(self, rhs: Sign) -> Self::Output {
                    self * rhs
                }
            }

            impl DivAssign<Sign> for $type {
                fn div_assign(&mut self, rhs: Sign) {
                    *self *= rhs
                }
            }
        )*
    };
}
sign_mul![i8, i16, i32, i64, i128, f32, f64];

impl Mul<Sign> for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Zero, _) | (_, Zero) => Zero,
            (Positive, Positive) | (Negative, Negative) => Positive,
            (Positive, Negative) | (Negative, Positive) => Negative,
        }
    }
}

impl MulAssign<Sign> for Sign {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div<Sign> for Sign {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl DivAssign<Sign> for Sign {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs
    }
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Not for Sign {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.negate()
    }
}

impl Sign {
    /// Return the opposite of the current sign.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::Sign;
    /// assert_eq!(Sign::Positive.negate(), Sign::Negative);
    /// assert_eq!(Sign::Negative.negate(), Sign::Positive);
    /// assert_eq!(Sign::Zero.negate(), Sign::Zero);
    /// ```
    pub fn negate(self) -> Self {
        match self {
            Positive => Negative,
            Negative => Positive,
            Zero => Zero,
        }
    }

    /// Is the sign positive?
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::Sign;
    /// assert!(Sign::Positive.is_positive());
    /// assert!(!Sign::Negative.is_positive());
    /// assert!(!Sign::Zero.is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        self as u8 == Positive as u8
    }

    /// Is the sign negative?
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::Sign;
    /// assert!(!Sign::Positive.is_negative());
    /// assert!(Sign::Negative.is_negative());
    /// assert!(!Sign::Zero.is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        self as u8 == Negative as u8
    }

    /// Is the value exactly zero?
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::Sign;
    /// assert!(!Sign::Positive.is_zero());
    /// assert!(!Sign::Negative.is_zero());
    /// assert!(Sign::Zero.is_zero());
    /// ```
    pub const fn is_zero(self) -> bool {
        self as u8 == Zero as u8
    }
}
