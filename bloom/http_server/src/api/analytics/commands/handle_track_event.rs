use crate::{
    api::{analytics::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use kernel::{domain::analytics::events, http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn handle_track_event(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::TrackEvent>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    // TODO
    let _input = input.into_inner();
    let service_input = events::TrackEvent {};
    ctx.analytics_service.handle_track_event(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
