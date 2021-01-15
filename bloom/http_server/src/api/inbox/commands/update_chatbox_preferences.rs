use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::UpdateChatboxPreferencesInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn update_chatbox_preferences(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UpdateChatboxPreferences>,
    actor: Actor,
) -> Result<api::Response<model::ChatboxPreferences>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UpdateChatboxPreferencesInput {};
    let chatbox_preferences = ctx
        .inbox_service
        .update_chatbox_preferences(actor, service_input)
        .await?;

    Ok(api::Response::ok(chatbox_preferences.into()))
}
