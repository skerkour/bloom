use crate::{
    api::{kernel::model::Success, newsletter::model::input},
    ServerContext,
};
use actix_web::web;
use inbox::service::UnsubscribeFromListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn unsubscribe_from_list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UnsubscribeFromList>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UnsubscribeFromListInput {
        subscription_id: input.subscription_id,
    };
    ctx.inbox_service.unsubscribe_from_list(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
