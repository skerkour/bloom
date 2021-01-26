use crate::{
    api::newsletter::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindNewsletterMessagesInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn messages(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetMessages>,
    actor: Actor,
) -> Result<api::Response<Vec<model::Message>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindNewsletterMessagesInput {
        namespace_id: input.namespace_id,
    };
    let messages = ctx.inbox_service.find_newsletter_messages(actor, service_input).await?;

    Ok(api::Response::ok(messages.into_iter().map(Into::into).collect()))
}
