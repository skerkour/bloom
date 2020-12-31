use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn setup_two_fa(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::EnableTwoFa>,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::EnableTwoFaInput { code: input.code };
    ctx.kernel_service.setup_two_fa(None, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
