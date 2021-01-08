use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn delete_group(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DeleteGroup>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::DeleteGroupInput {
        group_id: input.group_id,
    };
    ctx.kernel_service.delete_group(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
