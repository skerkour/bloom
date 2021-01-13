use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::CreateContactInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn create_contact(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CreateContact>,
    actor: Actor,
) -> Result<api::Response<model::Contact>, kernel::Error> {
    let input = input.into_inner();
    let service_input = CreateContactInput {
    };
    let contact = ctx.inbox_service.create_contact(actor, service_input).await?;

    Ok(api::Response::ok(contact.into()))
}
