use crate::{
    api::{
        inbox::model::{self, input},
        kernel::model::Success,
    },
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
    let service_input = SubscribeToListInput {};
    ctx.inbox_service.subscribe_to_list(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
