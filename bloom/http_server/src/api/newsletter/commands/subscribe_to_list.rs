use crate::{
    api::{kernel::model::Success, newsletter::model::input},
    ServerContext,
};
use actix_web::web;
use inbox::service::SubscribeToListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn subscribe_to_list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SubscribeToList>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = SubscribeToListInput {
        name: input.name,
        email: input.email,
        list_id: input.list_id,
    };
    ctx.inbox_service.subscribe_to_list(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
