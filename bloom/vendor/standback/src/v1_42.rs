use core::mem::ManuallyDrop;
use core::ptr;
#[cfg(feature = "std")]
use std::{
    sync::{Condvar, LockResult, MutexGuard, WaitTimeoutResult},
    time::{Duration, Instant},
};

use crate::traits::Sealed;

#[cfg(feature = "std")]
#[inline(always)]
fn new_wait_timeout_result(value: bool) -> WaitTimeoutResult {
    unsafe { core::mem::transmute(value) }
}

#[cfg(feature = "std")]
pub trait Condvar_v1_42: Sealed<Condvar> {
    fn wait_while<'a, T, F>(
        &self,
        guard: MutexGuard<'a, T>,
        condition: F,
    ) -> LockResult<MutexGuard<'a, T>>
    where
        F: FnMut(&mut T) -> bool;
    fn wait_timeout_while<'a, T, F>(
        &self,
        guard: MutexGuard<'a, T>,
        dur: Duration,
        condition: F,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>
    where
        F: FnMut(&mut T) -> bool;
}

#[cfg(feature = "std")]
impl Condvar_v1_42 for Condvar {
    fn wait_while<'a, T, F>(
        &self,
        mut guard: MutexGuard<'a, T>,
        mut condition: F,
    ) -> LockResult<MutexGuard<'a, T>>
    where
        F: FnMut(&mut T) -> bool,
    {
        while condition(&mut *guard) {
            guard = self.wait(guard)?;
        }
        Ok(guard)
    }

    fn wait_timeout_while<'a, T, F>(
        &self,
        mut guard: MutexGuard<'a, T>,
        dur: Duration,
        mut condition: F,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>
    where
        F: FnMut(&mut T) -> bool,
    {
        let start = Instant::now();
        loop {
            if !condition(&mut *guard) {
                return Ok((guard, new_wait_timeout_result(false)));
            }
            let timeout = match dur.checked_sub(start.elapsed()) {
                Some(timeout) => timeout,
                None => return Ok((guard, new_wait_timeout_result(true))),
            };
            guard = self.wait_timeout(guard, timeout)?.0;
        }
    }
}

pub trait ManuallyDrop_v1_42<T>: Sealed<ManuallyDrop<T>> {
    unsafe fn take(slot: &mut ManuallyDrop<T>) -> T;
}

impl<T> ManuallyDrop_v1_42<T> for ManuallyDrop<T> {
    #[must_use = "if you don't need the value, you can use `ManuallyDrop::drop` instead"]
    #[inline]
    unsafe fn take(slot: &mut ManuallyDrop<T>) -> T {
        ptr::read(slot as *mut _ as *const _)
    }
}

#[macro_export]
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+) => {
        match $expression {
            $( $pattern )|+ => true,
            _ => false,
        }
    };

    ($expression:expr, $( $pattern:pat )|+ if $guard:expr) => {
        match $expression {
            $( $pattern )|+ if $guard => true,
            _ => false
        }
    };

    ($expression:expr, $( $pattern:pat )|+ if $guard:expr ,) => {
        match $expression {
            $( $pattern )|+ if $guard => true,
            _ => false
        }
    };
}
