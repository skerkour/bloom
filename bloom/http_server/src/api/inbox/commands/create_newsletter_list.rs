use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::CreateNewsletterListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn create_newsletter_list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CreateNewsletterList>,
    actor: Actor,
) -> Result<api::Response<model::NewsletterList>, kernel::Error> {
    let input = input.into_inner();
    let service_input = CreateNewsletterListInput {
    };
    let list = ctx.inbox_service.create_newsletter_list(actor, service_input).await?;

    Ok(api::Response::ok(list.into()))
}
