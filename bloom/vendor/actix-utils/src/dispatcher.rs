//! Framed dispatcher service and related utilities

#![allow(type_alias_bounds)]

use std::pin::Pin;
use std::task::{Context, Poll};
use std::{fmt, mem};

use actix_codec::{AsyncRead, AsyncWrite, Decoder, Encoder, Framed};
use actix_service::{IntoService, Service};
use futures_util::{future::Future, stream::Stream, FutureExt};
use log::debug;

use crate::mpsc;

/// Framed transport errors
pub enum DispatcherError<E, U: Encoder<I> + Decoder, I> {
    Service(E),
    Encoder(<U as Encoder<I>>::Error),
    Decoder(<U as Decoder>::Error),
}

impl<E, U: Encoder<I> + Decoder, I> From<E> for DispatcherError<E, U, I> {
    fn from(err: E) -> Self {
        DispatcherError::Service(err)
    }
}

impl<E, U: Encoder<I> + Decoder, I> fmt::Debug for DispatcherError<E, U, I>
where
    E: fmt::Debug,
    <U as Encoder<I>>::Error: fmt::Debug,
    <U as Decoder>::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DispatcherError::Service(ref e) => write!(fmt, "DispatcherError::Service({:?})", e),
            DispatcherError::Encoder(ref e) => write!(fmt, "DispatcherError::Encoder({:?})", e),
            DispatcherError::Decoder(ref e) => write!(fmt, "DispatcherError::Decoder({:?})", e),
        }
    }
}

impl<E, U: Encoder<I> + Decoder, I> fmt::Display for DispatcherError<E, U, I>
where
    E: fmt::Display,
    <U as Encoder<I>>::Error: fmt::Debug,
    <U as Decoder>::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DispatcherError::Service(ref e) => write!(fmt, "{}", e),
            DispatcherError::Encoder(ref e) => write!(fmt, "{:?}", e),
            DispatcherError::Decoder(ref e) => write!(fmt, "{:?}", e),
        }
    }
}

pub enum Message<T> {
    Item(T),
    Close,
}

/// Dispatcher is a future that reads frames from Framed object
/// and passes them to the service.
#[pin_project::pin_project]
pub struct Dispatcher<S, T, U, I>
where
    S: Service<Request = <U as Decoder>::Item, Response = I>,
    S::Error: 'static,
    S::Future: 'static,
    T: AsyncRead + AsyncWrite,
    U: Encoder<I> + Decoder,
    I: 'static,
    <U as Encoder<I>>::Error: std::fmt::Debug,
{
    service: S,
    state: State<S, U, I>,
    #[pin]
    framed: Framed<T, U>,
    rx: mpsc::Receiver<Result<Message<I>, S::Error>>,
    tx: mpsc::Sender<Result<Message<I>, S::Error>>,
}

enum State<S: Service, U: Encoder<I> + Decoder, I> {
    Processing,
    Error(DispatcherError<S::Error, U, I>),
    FramedError(DispatcherError<S::Error, U, I>),
    FlushAndStop,
    Stopping,
}

impl<S: Service, U: Encoder<I> + Decoder, I> State<S, U, I> {
    fn take_error(&mut self) -> DispatcherError<S::Error, U, I> {
        match mem::replace(self, State::Processing) {
            State::Error(err) => err,
            _ => panic!(),
        }
    }

    fn take_framed_error(&mut self) -> DispatcherError<S::Error, U, I> {
        match mem::replace(self, State::Processing) {
            State::FramedError(err) => err,
            _ => panic!(),
        }
    }
}

impl<S, T, U, I> Dispatcher<S, T, U, I>
where
    S: Service<Request = <U as Decoder>::Item, Response = I>,
    S::Error: 'static,
    S::Future: 'static,
    T: AsyncRead + AsyncWrite,
    U: Decoder + Encoder<I>,
    I: 'static,
    <U as Decoder>::Error: std::fmt::Debug,
    <U as Encoder<I>>::Error: std::fmt::Debug,
{
    pub fn new<F: IntoService<S>>(framed: Framed<T, U>, service: F) -> Self {
        let (tx, rx) = mpsc::channel();
        Dispatcher {
            framed,
            rx,
            tx,
            service: service.into_service(),
            state: State::Processing,
        }
    }

    /// Construct new `Dispatcher` instance with customer `mpsc::Receiver`
    pub fn with_rx<F: IntoService<S>>(
        framed: Framed<T, U>,
        service: F,
        rx: mpsc::Receiver<Result<Message<I>, S::Error>>,
    ) -> Self {
        let tx = rx.sender();
        Dispatcher {
            framed,
            rx,
            tx,
            service: service.into_service(),
            state: State::Processing,
        }
    }

    /// Get sink
    pub fn get_sink(&self) -> mpsc::Sender<Result<Message<I>, S::Error>> {
        self.tx.clone()
    }

    /// Get reference to a service wrapped by `Dispatcher` instance.
    pub fn get_ref(&self) -> &S {
        &self.service
    }

    /// Get mutable reference to a service wrapped by `Dispatcher` instance.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.service
    }

    /// Get reference to a framed instance wrapped by `Dispatcher`
    /// instance.
    pub fn get_framed(&self) -> &Framed<T, U> {
        &self.framed
    }

    /// Get mutable reference to a framed instance wrapped by `Dispatcher` instance.
    pub fn get_framed_mut(&mut self) -> &mut Framed<T, U> {
        &mut self.framed
    }

    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> bool
    where
        S: Service<Request = <U as Decoder>::Item, Response = I>,
        S::Error: 'static,
        S::Future: 'static,
        T: AsyncRead + AsyncWrite,
        U: Decoder + Encoder<I>,
        I: 'static,
        <U as Encoder<I>>::Error: std::fmt::Debug,
    {
        loop {
            let this = self.as_mut().project();
            match this.service.poll_ready(cx) {
                Poll::Ready(Ok(_)) => {
                    let item = match this.framed.next_item(cx) {
                        Poll::Ready(Some(Ok(el))) => el,
                        Poll::Ready(Some(Err(err))) => {
                            *this.state = State::FramedError(DispatcherError::Decoder(err));
                            return true;
                        }
                        Poll::Pending => return false,
                        Poll::Ready(None) => {
                            *this.state = State::Stopping;
                            return true;
                        }
                    };

                    let tx = this.tx.clone();
                    actix_rt::spawn(this.service.call(item).map(move |item| {
                        let _ = tx.send(item.map(Message::Item));
                    }));
                }
                Poll::Pending => return false,
                Poll::Ready(Err(err)) => {
                    *this.state = State::Error(DispatcherError::Service(err));
                    return true;
                }
            }
        }
    }

    /// write to framed object
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> bool
    where
        S: Service<Request = <U as Decoder>::Item, Response = I>,
        S::Error: 'static,
        S::Future: 'static,
        T: AsyncRead + AsyncWrite,
        U: Decoder + Encoder<I>,
        I: 'static,
        <U as Encoder<I>>::Error: std::fmt::Debug,
    {
        loop {
            let mut this = self.as_mut().project();
            while !this.framed.is_write_buf_full() {
                match Pin::new(&mut this.rx).poll_next(cx) {
                    Poll::Ready(Some(Ok(Message::Item(msg)))) => {
                        if let Err(err) = this.framed.as_mut().write(msg) {
                            *this.state = State::FramedError(DispatcherError::Encoder(err));
                            return true;
                        }
                    }
                    Poll::Ready(Some(Ok(Message::Close))) => {
                        *this.state = State::FlushAndStop;
                        return true;
                    }
                    Poll::Ready(Some(Err(err))) => {
                        *this.state = State::Error(DispatcherError::Service(err));
                        return true;
                    }
                    Poll::Ready(None) | Poll::Pending => break,
                }
            }

            if !this.framed.is_write_buf_empty() {
                match this.framed.flush(cx) {
                    Poll::Pending => break,
                    Poll::Ready(Ok(_)) => (),
                    Poll::Ready(Err(err)) => {
                        debug!("Error sending data: {:?}", err);
                        *this.state = State::FramedError(DispatcherError::Encoder(err));
                        return true;
                    }
                }
            } else {
                break;
            }
        }

        false
    }
}

impl<S, T, U, I> Future for Dispatcher<S, T, U, I>
where
    S: Service<Request = <U as Decoder>::Item, Response = I>,
    S::Error: 'static,
    S::Future: 'static,
    T: AsyncRead + AsyncWrite,
    U: Decoder + Encoder<I>,
    I: 'static,
    <U as Encoder<I>>::Error: std::fmt::Debug,
    <U as Decoder>::Error: std::fmt::Debug,
{
    type Output = Result<(), DispatcherError<S::Error, U, I>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let this = self.as_mut().project();

            return match this.state {
                State::Processing => {
                    if self.as_mut().poll_read(cx) || self.as_mut().poll_write(cx) {
                        continue;
                    } else {
                        Poll::Pending
                    }
                }
                State::Error(_) => {
                    // flush write buffer
                    if !this.framed.is_write_buf_empty() {
                        if let Poll::Pending = this.framed.flush(cx) {
                            return Poll::Pending;
                        }
                    }
                    Poll::Ready(Err(this.state.take_error()))
                }
                State::FlushAndStop => {
                    if !this.framed.is_write_buf_empty() {
                        match this.framed.flush(cx) {
                            Poll::Ready(Err(err)) => {
                                debug!("Error sending data: {:?}", err);
                                Poll::Ready(Ok(()))
                            }
                            Poll::Pending => Poll::Pending,
                            Poll::Ready(Ok(_)) => Poll::Ready(Ok(())),
                        }
                    } else {
                        Poll::Ready(Ok(()))
                    }
                }
                State::FramedError(_) => Poll::Ready(Err(this.state.take_framed_error())),
                State::Stopping => Poll::Ready(Ok(())),
            };
        }
    }
}
