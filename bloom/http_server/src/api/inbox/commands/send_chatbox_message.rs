use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::SendChatboxMessageInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn send_chatbox_message(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SendChatboxMessage>,
    actor: Actor,
) -> Result<api::Response<model::ChatboxMessage>, kernel::Error> {
    let input = input.into_inner();
    let service_input = SendChatboxMessageInput {
        namespace_id: input.namespace_id,
        body: input.body,
    };
    let message = ctx.inbox_service.send_chatbox_message(actor, service_input).await?;

    Ok(api::Response::ok(message.into()))
}
