use core::marker::PhantomData;
use core::{fmt, mem};

use crate::marker::Unpin;

#[derive(PartialEq, Debug)]
pub struct RawWaker {
    data: *const (),
    vtable: &'static RawWakerVTable,
}

impl RawWaker {
    pub const fn new(data: *const (), vtable: &'static RawWakerVTable) -> RawWaker {
        RawWaker { data, vtable }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct RawWakerVTable {
    clone: unsafe fn(*const ()) -> RawWaker,
    wake: unsafe fn(*const ()),
    wake_by_ref: unsafe fn(*const ()),
    drop: unsafe fn(*const ()),
}

impl RawWakerVTable {
    pub fn new(
        clone: unsafe fn(*const ()) -> RawWaker,
        wake: unsafe fn(*const ()),
        wake_by_ref: unsafe fn(*const ()),
        drop: unsafe fn(*const ()),
    ) -> Self {
        Self {
            clone,
            wake,
            wake_by_ref,
            drop,
        }
    }
}

pub struct Context<'a> {
    waker: &'a Waker,
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}

impl<'a> Context<'a> {
    #[inline]
    pub fn from_waker(waker: &'a Waker) -> Self {
        Context {
            waker,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn waker(&self) -> &'a Waker {
        &self.waker
    }
}

impl fmt::Debug for Context<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("waker", &self.waker)
            .finish()
    }
}

#[repr(transparent)]
pub struct Waker {
    waker: RawWaker,
}

impl Unpin for Waker {}
unsafe impl Send for Waker {}
unsafe impl Sync for Waker {}

impl Waker {
    #[inline]
    pub fn wake(self) {
        let wake = self.waker.vtable.wake;
        let data = self.waker.data;

        mem::forget(self);

        unsafe { (wake)(data) };
    }

    #[inline]
    pub fn wake_by_ref(&self) {
        unsafe { (self.waker.vtable.wake_by_ref)(self.waker.data) }
    }

    #[inline]
    pub fn will_wake(&self, other: &Waker) -> bool {
        self.waker == other.waker
    }

    #[inline]
    pub unsafe fn from_raw(waker: RawWaker) -> Waker {
        Waker { waker }
    }
}

impl Clone for Waker {
    #[inline]
    fn clone(&self) -> Self {
        Waker {
            waker: unsafe { (self.waker.vtable.clone)(self.waker.data) },
        }
    }
}

impl Drop for Waker {
    #[inline]
    fn drop(&mut self) {
        unsafe { (self.waker.vtable.drop)(self.waker.data) }
    }
}

impl fmt::Debug for Waker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vtable_ptr = self.waker.vtable as *const RawWakerVTable;
        f.debug_struct("Waker")
            .field("data", &self.waker.data)
            .field("vtable", &vtable_ptr)
            .finish()
    }
}
