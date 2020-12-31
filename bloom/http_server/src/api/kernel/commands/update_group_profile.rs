use crate::{
    api::kernel::model::{input, Group},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn update_group_profile(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UpdateGroupProfile>,
) -> Result<api::Response<Group>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::UpdateGroupProfileInput {
        group_id: input.group_id,
        name: input.name,
        path: input.path,
        description: input.description,
    };
    let group = ctx.kernel_service.update_group_profile(None, service_input).await?;

    Ok(api::Response::ok(group.into()))
}
