use actix_service::Service;
use criterion::{criterion_main, Criterion};
use futures_util::future::join_all;
use futures_util::future::{ok, Ready};
use std::cell::{RefCell, UnsafeCell};
use std::rc::Rc;
use std::task::{Context, Poll};

struct SrvUC(Rc<UnsafeCell<usize>>);

impl Default for SrvUC {
    fn default() -> Self {
        Self(Rc::new(UnsafeCell::new(0)))
    }
}

impl Clone for SrvUC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Service for SrvUC {
    type Request = ();
    type Response = usize;
    type Error = ();
    type Future = Ready<Result<Self::Response, ()>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: ()) -> Self::Future {
        unsafe { *(*self.0).get() = *(*self.0).get() + 1 };
        ok(unsafe { *self.0.get() })
    }
}

struct SrvRC(Rc<RefCell<usize>>);

impl Default for SrvRC {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(0)))
    }
}

impl Clone for SrvRC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Service for SrvRC {
    type Request = ();
    type Response = usize;
    type Error = ();
    type Future = Ready<Result<Self::Response, ()>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: ()) -> Self::Future {
        let prev = *self.0.borrow();
        *(*self.0).borrow_mut() = prev + 1;
        ok(*self.0.borrow())
    }
}

/// Criterion Benchmark for async Service
/// Should be used from within criterion group:
/// ```rust,ignore
/// let mut criterion: ::criterion::Criterion<_> =
///     ::criterion::Criterion::default().configure_from_args();
/// bench_async_service(&mut criterion, ok_service(), "async_service_direct");
/// ```
///
/// Usable for benching Service wrappers:
/// Using minimum service code implementation we first measure
/// time to run minimum service, then measure time with wrapper.
///
/// Sample output
/// async_service_direct    time:   [1.0908 us 1.1656 us 1.2613 us]
pub fn bench_async_service<S>(c: &mut Criterion, srv: S, name: &str)
where
    S: Service<Request = (), Response = usize, Error = ()> + Clone + 'static,
{
    let mut rt = actix_rt::System::new("test");

    // start benchmark loops
    c.bench_function(name, move |b| {
        b.iter_custom(|iters| {
            let mut srvs: Vec<_> = (1..iters).map(|_| srv.clone()).collect();
            // exclude request generation, it appears it takes significant time vs call (3us vs 1us)
            let start = std::time::Instant::now();
            // benchmark body
            rt.block_on(async move { join_all(srvs.iter_mut().map(|srv| srv.call(()))).await });
            // check that at least first request succeeded
            start.elapsed()
        })
    });
}

pub fn service_benches() {
    let mut criterion: ::criterion::Criterion<_> =
        ::criterion::Criterion::default().configure_from_args();
    bench_async_service(&mut criterion, SrvUC::default(), "Service with UnsafeCell");
    bench_async_service(&mut criterion, SrvRC::default(), "Service with RefCell");
    bench_async_service(&mut criterion, SrvUC::default(), "Service with UnsafeCell");
    bench_async_service(&mut criterion, SrvRC::default(), "Service with RefCell");
}
criterion_main!(service_benches);
