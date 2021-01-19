use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn disable_two_fa(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DisableTwoFa>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::DisableTwoFaInput {
        code: input.code,
    };
    ctx.kernel_service.disable_two_fa(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
