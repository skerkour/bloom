use crate::{
    api::{inbox::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use inbox::service::DeleteNewsletterListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn delete_newsletter_list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DeleteNewsletterList>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = DeleteNewsletterListInput {};
    ctx.inbox_service.delete_newsletter_list(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
