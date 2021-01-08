use crate::{
    api::{analytics::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use kernel::{domain::analytics::events, http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn handle_page_event(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::PageEvent>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    // TODO
    let _input = input.into_inner();
    let service_input = events::PageEvent {};
    ctx.analytics_service.handle_page_event(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
