use crate::{
    api::newsletter::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::UpdateNewsletterListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn update_list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UpdateList>,
    actor: Actor,
) -> Result<api::Response<model::List>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UpdateNewsletterListInput {
        list_id: input.list_id,
        name: input.name,
        description: input.description,
    };
    let list = ctx.inbox_service.update_newsletter_list(actor, service_input).await?;

    Ok(api::Response::ok(list.into()))
}
