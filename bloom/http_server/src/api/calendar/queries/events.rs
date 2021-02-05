use crate::{api::calendar::model, ServerContext};
use actix_web::web;
use calendar::service;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn events(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::Events>,
    actor: Actor,
) -> Result<api::Response<Vec<model::Event>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::FindEventsInput {
        namespace_id: input.namespace_id,
        start_at: input.start_at,
        end_at: input.end_at,
    };
    let events = ctx.calendar_service.find_events(actor, service_input).await?;

    Ok(api::Response::ok(events.into_iter().map(Into::into).collect()))
}
