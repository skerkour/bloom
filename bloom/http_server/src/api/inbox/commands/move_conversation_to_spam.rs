use crate::{
    api::{inbox::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn move_conversation_to_spam(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::MoveConversation>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    ctx.inbox_service
        .move_conversation_to_spam(actor, input.conversation_id)
        .await?;

    Ok(api::Response::ok(true.into()))
}
