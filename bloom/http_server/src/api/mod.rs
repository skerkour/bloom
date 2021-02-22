use ::kernel::http::api;
use actix_web::{web::Json, Responder};
use std::collections::HashMap;

pub mod analytics;
pub mod avatars;
pub mod calendar;
pub mod files;
pub mod inbox;
pub mod kernel;
pub mod newsletter;
pub mod scalars;

/// 404 handler for the `/api` routes
pub async fn p404() -> Result<Json<()>, ::kernel::Error> {
    Err(::kernel::Error::NotFound(String::from("Route not found.")))
}

pub async fn index() -> impl Responder {
    let mut data = HashMap::new();
    data.insert("hello", "world");
    api::Response::ok(data)
}
