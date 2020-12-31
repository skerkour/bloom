use core::mem::ManuallyDrop;

#[derive(Copy)]
pub union MaybeUninit<T: Copy> {
    uninit: (),
    value: ManuallyDrop<T>,
}

impl<T: Copy> Clone for MaybeUninit<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> MaybeUninit<T> {
    #[inline(always)]
    pub fn new(val: T) -> MaybeUninit<T> {
        MaybeUninit {
            value: ManuallyDrop::new(val),
        }
    }

    #[inline(always)]
    pub fn uninit() -> MaybeUninit<T> {
        MaybeUninit { uninit: () }
    }

    #[inline]
    pub fn zeroed() -> MaybeUninit<T> {
        let mut u = MaybeUninit::<T>::uninit();
        unsafe {
            u.as_mut_ptr().write_bytes(0u8, 1);
        }
        u
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        unsafe { &*self.value as *const T }
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        unsafe { &mut *self.value as *mut T }
    }

    #[inline(always)]
    pub unsafe fn assume_init(self) -> T {
        ManuallyDrop::into_inner(self.value)
    }
}
