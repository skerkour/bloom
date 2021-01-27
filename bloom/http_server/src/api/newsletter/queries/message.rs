use crate::{
    api::newsletter::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindNewsletterMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetMessage>,
    actor: Actor,
) -> Result<api::Response<model::MessageWithLists>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindNewsletterMessageInput {
        message_id: input.message_id,
    };
    let message = ctx.inbox_service.find_newsletter_message(actor, service_input).await?;

    Ok(api::Response::ok(message.into()))
}
