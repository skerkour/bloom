use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindChatboxPreferencesInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn chatbox_preferences(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetChatboxPreferences>,
    actor: Actor,
) -> Result<api::Response<model::ChatboxPreferences>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindChatboxPreferencesInput {
        namespace_id: input.namespace_id,
    };
    let preferences = ctx.inbox_service.find_chatbox_preferences(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_chatbox_preferences(
        &ctx.kernel_service,
        preferences,
    )))
}
