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
    let service_input = UpdateChatboxPreferencesInput {
        namespace_id: input.namespace_id,
        color: input.color,
        name: input.name,
        show_branding: input.show_branding,
        welcome_message: input.welcome_message,
        twitter: input.twitter,
        facebook_url: input.facebook_url,
        instagram: input.instagram,
        whatsapp_number: input.whatsapp_number,
        mastodon_url: input.mastodon_url,
        website_url: input.website_url,
        telegram: input.telegram,
    };
    let chatbox_preferences = ctx
        .inbox_service
        .update_chatbox_preferences(actor, service_input)
        .await?;

    Ok(api::Response::ok(model::convert_chatbox_preferences(
        &ctx.kernel_service,
        chatbox_preferences,
    )))
}
