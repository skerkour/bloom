use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn update_group_profile(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::UpdateGroupProfile>,
    actor: Actor,
) -> Result<api::Response<model::Group>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::UpdateGroupProfileInput {
        group_id: input.group_id,
        name: input.name,
        path: input.path,
        description: input.description,
    };
    let group = ctx.kernel_service.update_group_profile(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_group(
        &ctx.kernel_service,
        group,
        true,
    )))
}
