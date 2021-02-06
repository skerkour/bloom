use crate::{
    api::{calendar::model, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use calendar::service;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn delete_event(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::DeleteEvent>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::DeleteEventInput {
        event_id: input.event_id,
    };
    ctx.calendar_service.delete_event(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
