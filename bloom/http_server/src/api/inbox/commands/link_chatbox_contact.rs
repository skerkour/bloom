use crate::{
    api::{inbox::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use inbox::service::LinkChatboxContactInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn link_chatbox_contact(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::LinkChatboxContact>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = LinkChatboxContactInput {
        namespace_id: input.namespace_id,
        email: input.email,
    };
    ctx.inbox_service.link_chatbox_contact(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
