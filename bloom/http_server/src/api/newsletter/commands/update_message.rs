use crate::{
    api::newsletter::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::UpdateNewsletterMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn update_message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UpdateMessage>,
    actor: Actor,
) -> Result<api::Response<model::MessageWithLists>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UpdateNewsletterMessageInput {
        message_id: input.message_id,
        list_id: input.list_id,
        name: input.name,
        subject: input.subject,
        body: input.body,
        scheduled_for: None,
    };
    let message = ctx
        .inbox_service
        .update_newsletter_message(actor, service_input)
        .await?;

    Ok(api::Response::ok(message.into()))
}
