use actix_service::boxed::BoxFuture;
use actix_service::IntoService;
use actix_service::Service;
/// Benchmark various implementations of and_then
use criterion::{criterion_main, Criterion};
use futures_util::future::join_all;
use futures_util::future::TryFutureExt;
use std::cell::{RefCell, UnsafeCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

/*
 * Test services A,B for AndThen service implementations
 */

async fn svc1(_: ()) -> Result<usize, ()> {
    Ok(1)
}

async fn svc2(req: usize) -> Result<usize, ()> {
    Ok(req + 1)
}

/*
 * AndThenUC - original AndThen service based on UnsafeCell
 * Cut down version of actix_service::AndThenService based on actix-service::Cell
 */

struct AndThenUC<A, B>(Rc<UnsafeCell<(A, B)>>);

impl<A, B> AndThenUC<A, B> {
    fn new(a: A, b: B) -> Self
    where
        A: Service,
        B: Service<Request = A::Response, Error = A::Error>,
    {
        Self(Rc::new(UnsafeCell::new((a, b))))
    }
}

impl<A, B> Clone for AndThenUC<A, B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A, B> Service for AndThenUC<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    type Request = A::Request;
    type Response = B::Response;
    type Error = A::Error;
    type Future = AndThenServiceResponse<A, B>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: A::Request) -> Self::Future {
        let fut = unsafe { &mut *(*self.0).get() }.0.call(req);
        AndThenServiceResponse {
            state: State::A(fut, Some(self.0.clone())),
        }
    }
}

#[pin_project::pin_project]
pub(crate) struct AndThenServiceResponse<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    #[pin]
    state: State<A, B>,
}

#[pin_project::pin_project(project = StateProj)]
enum State<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    A(#[pin] A::Future, Option<Rc<UnsafeCell<(A, B)>>>),
    B(#[pin] B::Future),
    Empty,
}

impl<A, B> Future for AndThenServiceResponse<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    type Output = Result<B::Response, A::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();

        match this.state.as_mut().project() {
            StateProj::A(fut, b) => match fut.poll(cx)? {
                Poll::Ready(res) => {
                    let b = b.take().unwrap();
                    this.state.set(State::Empty); // drop fut A
                    let fut = unsafe { &mut (*b.get()).1 }.call(res);
                    this.state.set(State::B(fut));
                    self.poll(cx)
                }
                Poll::Pending => Poll::Pending,
            },
            StateProj::B(fut) => fut.poll(cx).map(|r| {
                this.state.set(State::Empty);
                r
            }),
            StateProj::Empty => {
                panic!("future must not be polled after it returned `Poll::Ready`")
            }
        }
    }
}

/*
 * AndThenRC - AndThen service based on RefCell
 */

struct AndThenRC<A, B>(Rc<RefCell<(A, B)>>);

impl<A, B> AndThenRC<A, B> {
    fn new(a: A, b: B) -> Self
    where
        A: Service,
        B: Service<Request = A::Response, Error = A::Error>,
    {
        Self(Rc::new(RefCell::new((a, b))))
    }
}

impl<A, B> Clone for AndThenRC<A, B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A, B> Service for AndThenRC<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    type Request = A::Request;
    type Response = B::Response;
    type Error = A::Error;
    type Future = AndThenServiceResponseRC<A, B>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: A::Request) -> Self::Future {
        let fut = self.0.borrow_mut().0.call(req);
        AndThenServiceResponseRC {
            state: StateRC::A(fut, Some(self.0.clone())),
        }
    }
}

#[pin_project::pin_project]
pub(crate) struct AndThenServiceResponseRC<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    #[pin]
    state: StateRC<A, B>,
}

#[pin_project::pin_project(project = StateRCProj)]
enum StateRC<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    A(#[pin] A::Future, Option<Rc<RefCell<(A, B)>>>),
    B(#[pin] B::Future),
    Empty,
}

impl<A, B> Future for AndThenServiceResponseRC<A, B>
where
    A: Service,
    B: Service<Request = A::Response, Error = A::Error>,
{
    type Output = Result<B::Response, A::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();

        match this.state.as_mut().project() {
            StateRCProj::A(fut, b) => match fut.poll(cx)? {
                Poll::Ready(res) => {
                    let b = b.take().unwrap();
                    this.state.set(StateRC::Empty); // drop fut A
                    let fut = b.borrow_mut().1.call(res);
                    this.state.set(StateRC::B(fut));
                    self.poll(cx)
                }
                Poll::Pending => Poll::Pending,
            },
            StateRCProj::B(fut) => fut.poll(cx).map(|r| {
                this.state.set(StateRC::Empty);
                r
            }),
            StateRCProj::Empty => {
                panic!("future must not be polled after it returned `Poll::Ready`")
            }
        }
    }
}

/*
 * AndThenRCFuture - AndThen service based on RefCell
 * and standard futures::future::and_then combinator in a Box
 */

struct AndThenRCFuture<A, B>(Rc<RefCell<(A, B)>>);

impl<A, B> AndThenRCFuture<A, B> {
    fn new(a: A, b: B) -> Self
    where
        A: Service,
        B: Service<Request = A::Response, Error = A::Error>,
    {
        Self(Rc::new(RefCell::new((a, b))))
    }
}

impl<A, B> Clone for AndThenRCFuture<A, B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A, B> Service for AndThenRCFuture<A, B>
where
    A: Service + 'static,
    A::Future: 'static,
    B: Service<Request = A::Response, Error = A::Error> + 'static,
    B::Future: 'static,
{
    type Request = A::Request;
    type Response = B::Response;
    type Error = A::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: A::Request) -> Self::Future {
        let fut = self.0.borrow_mut().0.call(req);
        let core = self.0.clone();
        let fut2 = move |res| (*core).borrow_mut().1.call(res);
        Box::pin(fut.and_then(fut2))
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
    bench_async_service(
        &mut criterion,
        AndThenUC::new(svc1.into_service(), svc2.into_service()),
        "AndThen with UnsafeCell",
    );
    bench_async_service(
        &mut criterion,
        AndThenRC::new(svc1.into_service(), svc2.into_service()),
        "AndThen with RefCell",
    );
    bench_async_service(
        &mut criterion,
        AndThenUC::new(svc1.into_service(), svc2.into_service()),
        "AndThen with UnsafeCell",
    );
    bench_async_service(
        &mut criterion,
        AndThenRC::new(svc1.into_service(), svc2.into_service()),
        "AndThen with RefCell",
    );
    bench_async_service(
        &mut criterion,
        AndThenRCFuture::new(svc1.into_service(), svc2.into_service()),
        "AndThen with RefCell via future::and_then",
    );
}

criterion_main!(service_benches);
