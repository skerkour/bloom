use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindContactInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn contact(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetContact>,
    actor: Actor,
) -> Result<api::Response<model::Contact>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindContactInput {
        contact_id: input.contact_id,
    };
    let contact = ctx.inbox_service.find_contact(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_contact(&ctx.kernel_service, contact)))
}
