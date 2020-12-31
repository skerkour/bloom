use crate::Duration;
use core::{
    cmp::{Ord, Ordering, PartialEq, PartialOrd},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};
use standback::convert::{TryFrom, TryInto};
use std::time::Instant as StdInstant;

/// A measurement of a monotonically non-decreasing clock. Opaque and useful
/// only with [`Duration`].
///
/// Instants are always guaranteed to be no less than any previously measured
/// instant when created, and are often useful for tasks such as measuring
/// benchmarks or timing how long an operation takes.
///
/// Note, however, that instants are not guaranteed to be **steady**. In other
/// words, each tick of the underlying clock may not be the same length (e.g.
/// some seconds may be longer than others). An instant may jump forwards or
/// experience time dilation (slow down or speed up), but it will never go
/// backwards.
///
/// Instants are opaque types that can only be compared to one another. There is
/// no method to get "the number of seconds" from an instant. Instead, it only
/// allows measuring the duration between two instants (or comparing two
/// instants).
///
/// This implementation allows for operations with signed [`Duration`]s, but is
/// otherwise identical to [`std::time::Instant`].
#[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    /// Inner representation, using `std::time::Instant`.
    inner: StdInstant,
}

impl Instant {
    /// Returns an `Instant` corresponding to "now".
    ///
    /// ```rust
    /// # use time::Instant;
    /// println!("{:?}", Instant::now());
    /// ```
    pub fn now() -> Self {
        Self {
            inner: StdInstant::now(),
        }
    }

    /// Returns the amount of time elapsed since this instant was created. The
    /// duration will always be nonnegative if the instant is not synthetically
    /// created.
    ///
    /// ```rust
    /// # use time::{Instant, prelude::*};
    /// # use std::thread;
    /// let instant = Instant::now();
    /// thread::sleep(1.std_milliseconds());
    /// assert!(instant.elapsed() >= 1.milliseconds());
    /// ```
    pub fn elapsed(self) -> Duration {
        Self::now() - self
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, prelude::*};
    /// let now = Instant::now();
    /// assert_eq!(
    ///     now.checked_add(5.seconds()),
    ///     Some(now + 5.seconds())
    /// );
    /// assert_eq!(
    ///     now.checked_add((-5).seconds()),
    ///     Some(now + (-5).seconds())
    /// );
    /// ```
    ///
    /// This function is only present when using rustc >= 1.34.0.
    #[cfg(__time_02_instant_checked_ops)]
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        if duration.is_zero() {
            Some(self)
        } else if duration.is_positive() {
            self.inner.checked_add(duration.abs_std()).map(From::from)
        } else {
            // duration.is_negative()
            self.inner.checked_sub(duration.abs_std()).map(From::from)
        }
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be
    /// represented as `Instant` (which means it's inside the bounds of the
    /// underlying data structure), `None` otherwise.
    ///
    /// ```rust
    /// # use time::{Instant, prelude::*};
    /// let now = Instant::now();
    /// assert_eq!(
    ///     now.checked_sub(5.seconds()),
    ///     Some(now - 5.seconds())
    /// );
    /// assert_eq!(
    ///     now.checked_sub((-5).seconds()),
    ///     Some(now - (-5).seconds())
    /// );
    /// ```
    ///
    /// This function is only present when using rustc >= 1.34.0.
    #[cfg(__time_02_instant_checked_ops)]
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        self.checked_add(-duration)
    }
}

#[allow(clippy::missing_docs_in_private_items)]
impl Instant {
    #[cfg(feature = "deprecated")]
    #[deprecated(since = "0.2.0", note = "Use `rhs - lhs`")]
    pub fn to(&self, later: Self) -> Duration {
        later - *self
    }
}

impl From<StdInstant> for Instant {
    fn from(instant: StdInstant) -> Self {
        Self { inner: instant }
    }
}

impl From<Instant> for StdInstant {
    fn from(instant: Instant) -> Self {
        instant.inner
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        match self.inner.cmp(&other.inner) {
            Ordering::Equal => Duration::zero(),
            Ordering::Greater => (self.inner - other.inner)
                .try_into()
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
            Ordering::Less => -Duration::try_from(other.inner - self.inner)
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
        }
    }
}

impl Sub<StdInstant> for Instant {
    type Output = Duration;

    fn sub(self, other: StdInstant) -> Self::Output {
        self - Self::from(other)
    }
}

impl Sub<Instant> for StdInstant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Self::Output {
        Instant::from(self) - other
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        if duration.is_positive() {
            (self.inner + duration.abs_std()).into()
        } else if duration.is_negative() {
            (self.inner - duration.abs_std()).into()
        } else {
            self
        }
    }
}

impl Add<Duration> for StdInstant {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        (Instant::from(self) + duration).into()
    }
}

impl Add<StdDuration> for Instant {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        Self {
            inner: self.inner + duration,
        }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<Duration> for StdInstant {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Instant {
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<Duration> for StdInstant {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        (Instant::from(self) - duration).into()
    }
}

impl Sub<StdDuration> for Instant {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        Self {
            inner: self.inner - duration,
        }
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<Duration> for StdInstant {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Instant {
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl PartialEq<StdInstant> for Instant {
    fn eq(&self, rhs: &StdInstant) -> bool {
        self.inner.eq(rhs)
    }
}

impl PartialEq<Instant> for StdInstant {
    fn eq(&self, rhs: &Instant) -> bool {
        self.eq(&rhs.inner)
    }
}

impl PartialOrd<StdInstant> for Instant {
    fn partial_cmp(&self, rhs: &StdInstant) -> Option<Ordering> {
        self.inner.partial_cmp(rhs)
    }
}

impl PartialOrd<Instant> for StdInstant {
    fn partial_cmp(&self, rhs: &Instant) -> Option<Ordering> {
        self.partial_cmp(&rhs.inner)
    }
}
