use crate::{api::calendar::model, ServerContext};
use actix_web::web;
use calendar::service;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn create_event(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::CreateEvent>,
    actor: Actor,
) -> Result<api::Response<model::Event>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CreateEventInput {
        namespace_id: input.namespace_id,
        title: input.title,
        description: input.description,
        location: input.location,
        start_at: input.start_at,
        end_at: input.end_at,
    };
    let event = ctx.calendar_service.create_event(actor, service_input).await?;

    Ok(api::Response::ok(event.into()))
}
