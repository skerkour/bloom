use crate::{
    api::{kernel::model::Success, newsletter::model::input},
    ServerContext,
};
use actix_web::web;
use inbox::service::DeleteNewsletterMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn delete_message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DeleteMessage>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = DeleteNewsletterMessageInput {
        message_id: input.message_id,
    };
    ctx.inbox_service
        .delete_newsletter_message(actor, service_input)
        .await?;

    Ok(api::Response::ok(true.into()))
}
