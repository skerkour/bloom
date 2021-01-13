use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindNewsletterMessagesInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn newsletter_messages(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetNewsletterMessages>,
    actor: Actor,
) -> Result<api::Response<Vec<model::NewsletterMessage>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindNewsletterMessagesInput {};
    let messages = ctx.inbox_service.find_newsletter_messages(actor, service_input).await?;

    Ok(api::Response::ok(messages.into_iter().map(Into::into).collect()))
}
