use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::UpdateNewsletterMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn update_newsletter_message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UpdateNewsletterMessage>,
    actor: Actor,
) -> Result<api::Response<model::NewsletterMessage>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UpdateNewsletterMessageInput {};
    let message = ctx
        .inbox_service
        .update_newsletter_message(actor, service_input)
        .await?;

    Ok(api::Response::ok(message.into()))
}
