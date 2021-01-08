use crate::{
    api::kernel::model::{Empty, SetupTwoFa},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn complete_two_fa_setup(
    ctx: web::Data<Arc<ServerContext>>,
    _input: Json<Empty>,
    actor: Actor,
) -> Result<api::Response<SetupTwoFa>, kernel::Error> {
    let base64_encoded_qr_code = ctx.kernel_service.complete_two_fa_setup(actor).await?;

    Ok(api::Response::ok(SetupTwoFa {
        base64_qr_code: base64_encoded_qr_code,
    }))
}
