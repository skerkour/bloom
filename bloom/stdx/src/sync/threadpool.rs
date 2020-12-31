use thiserror::Error;

/// Runs the provided closure on a thread where blocking is acceptable.
///
/// It can be any thread other than tokio's worker threads: tokio blocking threads or a distinct
/// threadpool.
///
/// It may or may not spwn a new OS thread.
/// If a thread is available in the threadpool, it is used
/// else if the number of spawned thread is less than the limit, a new OS thread is spawned,
/// otherwise the task is queued.
///
/// This function is intended for non-async operations that eventually
/// finish on their own.
/// Closures spawned using `spawn` cannot be cancelled.
///
/// # Examples
///
/// ```
/// use stdx::sync::threadpool;
///
/// # async fn docs() -> Result<(), Box<dyn std::error::Error>>{
/// let res = threadpool::spawn_blocking(move || {
///     // do some compute-heavy work or call synchronous code
///     "done computing"
/// }).await?;
///
/// assert_eq!(res, "done computing");
/// # Ok(())
/// # }
/// ```
pub async fn spawn_blocking<F, R>(f: F) -> Result<R, Error>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    // tokio have a dedicated threadpool for blocking tasks.
    // It spawns new threads until a limit when no thread is available.
    // When a blocking thread is not used for an amount of time, tokio kill it.
    // See https://github.com/tokio-rs/tokio/blob/master/tokio/src/runtime/blocking/pool.rs
    // and https://github.com/tokio-rs/tokio/blob/master/tokio/src/runtime/handle.rs
    // for details.
    // If we encounter limits due to the 'unbounded' (actually bounded to 512 by default) nature
    // of tokio's blocking threadpool, we me try to swtich to a fixed-size threadpool like
    // `rayon` or `threadpool`
    tokio::task::spawn_blocking(f).await.map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("threadpool: JoinError")]
    JoinError,
}

impl std::convert::From<tokio::task::JoinError> for Error {
    fn from(_: tokio::task::JoinError) -> Self {
        Error::JoinError
    }
}
