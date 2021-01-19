use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::SendMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn send_message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SendMessage>,
    actor: Actor,
) -> Result<api::Response<model::Message>, kernel::Error> {
    let input = input.into_inner();
    let service_input = SendMessageInput {
        conversation_id: input.conversation_id,
        body: input.body,
    };
    let message = ctx.inbox_service.send_message(actor, service_input).await?;

    Ok(api::Response::ok(message.into()))
}
