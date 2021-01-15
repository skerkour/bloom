use crate::{
    api::{inbox::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use inbox::service::DeleteNewsletterMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn delete_newsletter_message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DeleteNewsletterMessage>,
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
