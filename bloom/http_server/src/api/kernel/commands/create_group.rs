use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn create_group(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::CreateGroup>,
    actor: Actor,
) -> Result<api::Response<model::Group>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CreateGroupInput {
        name: input.name,
        path: input.path,
        description: input.description,
    };
    let group = ctx.kernel_service.create_group(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_group(
        &ctx.kernel_service,
        group,
        true,
    )))
}
