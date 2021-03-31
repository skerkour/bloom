use core::ops::Range;

use crate::traits::Sealed;

#[cfg(__standback_since_1_36)]
pub(crate) mod future {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    pub struct Ready<T>(Option<T>);

    impl<T> Unpin for Ready<T> {}

    impl<T> Future for Ready<T> {
        type Output = T;

        #[inline]
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
            Poll::Ready(self.0.take().expect("Ready polled after completion"))
        }
    }

    pub fn ready<T>(t: T) -> Ready<T> {
        Ready(Some(t))
    }

    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Pending<T> {
        _data: core::marker::PhantomData<T>,
    }

    pub fn pending<T>() -> Pending<T> {
        Pending {
            _data: core::marker::PhantomData,
        }
    }

    impl<T> Future for Pending<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<T> {
            Poll::Pending
        }
    }

    impl<T> Unpin for Pending<T> {}

    impl<T> Clone for Pending<T> {
        fn clone(&self) -> Self {
            pending()
        }
    }
}

pub trait Slice_v1_48<T>: Sealed<[T]> {
    fn as_ptr_range(&self) -> Range<*const T>;
    fn as_mut_ptr_range(&mut self) -> Range<*mut T>;
}

impl<T> Slice_v1_48<T> for [T] {
    #[inline]
    fn as_ptr_range(&self) -> Range<*const T> {
        let start = self.as_ptr();
        let end = unsafe { start.add(self.len()) };
        start..end
    }

    #[inline]
    fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        let start = self.as_mut_ptr();
        let end = unsafe { start.add(self.len()) };
        start..end
    }
}
