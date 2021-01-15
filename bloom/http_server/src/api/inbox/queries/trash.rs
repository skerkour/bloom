use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindTrashInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn trash(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetTrash>,
    actor: Actor,
) -> Result<api::Response<Vec<model::Conversation>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindTrashInput {
        namespace_id: input.namespace_id,
    };
    let conversations = ctx.inbox_service.find_trash(actor, service_input).await?;

    Ok(api::Response::ok(conversations.into_iter().map(Into::into).collect()))
}
