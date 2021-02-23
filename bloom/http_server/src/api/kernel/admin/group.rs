use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn group(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::AdminGetGroup>,
    actor: Actor,
) -> Result<api::Response<model::Group>, kernel::Error> {
    let input = input.into_inner();
    let group = ctx.kernel_service.admin_find_group(actor, input.group_id).await?;

    Ok(api::Response::ok(model::convert_group(
        &ctx.kernel_service,
        group,
        true,
    )))
}
