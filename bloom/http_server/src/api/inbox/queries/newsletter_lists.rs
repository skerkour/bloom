use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindNewsletterListsInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn newsletter_lists(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetNewsletterLists>,
    actor: Actor,
) -> Result<api::Response<Vec<model::NewsletterList>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindNewsletterListsInput {};
    let lists = ctx.inbox_service.find_newsletter_lists(actor, service_input).await?;

    Ok(api::Response::ok(lists.into_iter().map(Into::into).collect()))
}
