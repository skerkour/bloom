use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn delete_my_account(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DeleteMyAccount>,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::DeleteMyAccountInput {
        two_fa_totp_code: input.two_fa_totp_code,
    };
    ctx.kernel_service.delete_my_account(None, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
