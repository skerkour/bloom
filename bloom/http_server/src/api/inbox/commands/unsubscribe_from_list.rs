use crate::{
    api::{inbox::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use inbox::service::UnsubscribeFromListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn unsubscribe_from_list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SubscribeToList>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UnsubscribeFromListInput {};
    ctx.inbox_service.unsubscribe_from_list(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
