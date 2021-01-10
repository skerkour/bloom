use crate::{
    api::{analytics::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn handle_page_event(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::PageEvent>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    ctx.analytics_service
        .handle_page_event(actor, input.into_inner())
        .await?;

    Ok(api::Response::ok(true.into()))
}
