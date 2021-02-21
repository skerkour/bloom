use crate::{
    api::newsletter::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindNewsletterListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetList>,
    actor: Actor,
) -> Result<api::Response<model::ListWithDetails>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindNewsletterListInput {
        list_id: input.list_id,
    };
    let list = ctx.inbox_service.find_newsletter_list(actor, service_input).await?;

    Ok(api::Response::ok(list.into()))
}
