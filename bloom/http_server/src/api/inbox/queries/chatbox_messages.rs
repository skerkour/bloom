use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindChatboxMessagesInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn chatbox_messages(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetChatboxMessages>,
    actor: Actor,
) -> Result<api::Response<Vec<model::ChatboxMessage>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindChatboxMessagesInput {
        namespace_id: input.namespace_id,
        after: input.after,
    };
    let messages = ctx.inbox_service.find_chatbox_messages(actor, service_input).await?;

    Ok(api::Response::ok(messages.into_iter().map(Into::into).collect()))
}
