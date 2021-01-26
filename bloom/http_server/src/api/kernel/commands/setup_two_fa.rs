use crate::{api::kernel::model::SetupTwoFa, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn setup_two_fa(
    ctx: web::Data<Arc<ServerContext>>,
    actor: Actor,
) -> Result<api::Response<SetupTwoFa>, kernel::Error> {
    let base64_encoded_qr_code = ctx.kernel_service.setup_two_fa(actor).await?;

    Ok(api::Response::ok(SetupTwoFa {
        base64_qr_code: base64_encoded_qr_code,
    }))
}
