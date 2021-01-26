use crate::{
    api::newsletter::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::SendTestNewsletterMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn send_test_message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SendMessage>,
    actor: Actor,
) -> Result<api::Response<model::Message>, kernel::Error> {
    let input = input.into_inner();
    let service_input = SendTestNewsletterMessageInput {
        message_id: input.message_id,
    };
    let message = ctx
        .inbox_service
        .send_test_newsletter_message(actor, service_input)
        .await?;

    Ok(api::Response::ok(message.into()))
}
