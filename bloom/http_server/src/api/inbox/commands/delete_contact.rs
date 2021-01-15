use crate::{
    api::{inbox::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use inbox::service::DeleteContactInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn delete_contact(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DeleteContact>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = DeleteContactInput {
        contact_id: input.contact_id,
    };
    ctx.inbox_service.delete_contact(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
