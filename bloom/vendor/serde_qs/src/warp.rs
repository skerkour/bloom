//! Functionality for using `serde_qs` with `warp`.
//!
//! Enable with the `warp` feature.

extern crate warp_framework as warp;

use crate::{de::Config as QsConfig, error};
use serde::de;
use std::sync::Arc;
use warp::{http::StatusCode, reject::Reject, Filter, Rejection, Reply};

impl Reject for error::Error {}

/// Extract typed information from from the request's query.
///
/// ## Example
///
/// ```rust
/// # extern crate warp_framework as warp;
/// # #[macro_use] extern crate serde_derive;
/// use warp::Filter;
/// use serde_qs::Config;
///
/// #[derive(Deserialize)]
/// pub struct UsersFilter {
///    id: Vec<u64>,
/// }
///
/// fn main() {
///     let filter = serde_qs::warp::query(Config::default())
///         .and_then(|info: UsersFilter| async move {
///             Ok::<_, warp::Rejection>(
///                 info.id.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")
///             )
///         })
///         .recover(serde_qs::warp::recover_fn);
/// }
/// ```
pub fn query<T>(config: QsConfig) -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: de::DeserializeOwned + Send + 'static,
{
    let config = Arc::new(config);

    warp::query::raw()
        .or_else(|_| async {
            tracing::debug!("route was called without a query string, defaulting to empty");

            Ok::<_, Rejection>((String::new(),))
        })
        .and_then(move |query: String| {
            let config = Arc::clone(&config);

            async move {
                config.deserialize_str(query.as_str()).map_err(|err| {
                    tracing::debug!("failed to decode query string '{}': {:?}", query, err);

                    Rejection::from(err)
                })
            }
        })
}

/// Use this as the function for a `.recover()` after assembled filter
///
/// This is not strictly required but changes the response from a
/// "500 Internal Server Error" to a "400 Bad Request"
pub async fn recover_fn(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = rejection.find::<error::Error>() {
        Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Err(rejection)
    }
}
