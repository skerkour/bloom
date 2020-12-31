//! The types of backoff strategies that are supported

use crate::RetryPolicy;
use std::time::Duration;

/// Trait for computing the amount of delay between attempts.
pub trait BackoffStrategy<E> {
    /// The delay type. Will normally be either [`Duration`] or [`RetryPolicy`].
    ///
    /// [`Duration`]: https://doc.rust-lang.org/stable/std/time/struct.Duration.html
    /// [`RetryPolicy`]: ../enum.RetryPolicy.html
    type Output;

    /// Compute the amount of delay given the number of attempts so far and the most previous
    /// error.
    fn delay(&mut self, attempt: u32, error: &E) -> Self::Output;
}

/// No backoff. This will make the future be retried immediately without any delay in between
/// attempts.
#[derive(Debug)]
pub struct NoBackoff;

impl<E> BackoffStrategy<E> for NoBackoff {
    type Output = Duration;

    #[inline]
    fn delay(&mut self, _attempt: u32, _error: &E) -> Duration {
        Duration::new(0, 0)
    }
}

/// Exponential backoff. The delay will double each time.
#[derive(Debug)]
pub struct ExponentialBackoff {
    pub(crate) delay: Duration,
}

impl<E> BackoffStrategy<E> for ExponentialBackoff {
    type Output = Duration;

    #[inline]
    fn delay(&mut self, _attempt: u32, _error: &E) -> Duration {
        let prev_delay = self.delay;
        self.delay *= 2;
        prev_delay
    }
}

/// Fixed backoff. The delay wont change between attempts.
#[derive(Debug)]
pub struct FixedBackoff {
    pub(crate) delay: Duration,
}

impl<E> BackoffStrategy<E> for FixedBackoff {
    type Output = Duration;

    #[inline]
    fn delay(&mut self, _attempt: u32, _error: &E) -> Duration {
        self.delay
    }
}

/// Linear backoff. The delay will scale linearly with the number of attempts.
#[derive(Debug)]
pub struct LinearBackoff {
    pub(crate) delay: Duration,
}

impl<E> BackoffStrategy<E> for LinearBackoff {
    type Output = Duration;

    #[inline]
    fn delay(&mut self, attempt: u32, _error: &E) -> Duration {
        self.delay * attempt
    }
}

/// A custom backoff strategy defined by a function.
#[derive(Debug)]
pub struct CustomBackoffStrategy<F> {
    pub(crate) f: F,
}

impl<F, E, T> BackoffStrategy<E> for CustomBackoffStrategy<F>
where
    F: FnMut(u32, &E) -> T,
    RetryPolicy: From<T>,
{
    type Output = RetryPolicy;

    #[inline]
    fn delay(&mut self, attempt: u32, error: &E) -> RetryPolicy {
        RetryPolicy::from((self.f)(attempt, error))
    }
}
