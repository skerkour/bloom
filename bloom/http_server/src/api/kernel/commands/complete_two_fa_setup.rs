use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn complete_two_fa_setup(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CompleteTwoFaSetup>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CompleteTwoFaSetup {
        code: input.code,
    };
    ctx.kernel_service.complete_two_fa_setup(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
