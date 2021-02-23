use crate::{
    api::kernel::model::{self, convert_group},
    ServerContext,
};
use actix_web::web::{self, Json};
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn group(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::GetGroup>,
    actor: Actor,
) -> Result<api::Response<model::Group>, kernel::Error> {
    let input = input.into_inner();

    let group = ctx.kernel_service.find_group(actor, input.path).await?;

    Ok(api::Response::ok(convert_group(&ctx.kernel_service, group, true)))
}
