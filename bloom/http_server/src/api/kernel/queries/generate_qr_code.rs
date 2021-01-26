use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn generate_qr_code(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::GenerateQrCode>,
    actor: Actor,
) -> Result<api::Response<model::QrCode>, kernel::Error> {
    let input = input.into_inner();
    let base64_jpeg_qr_code = ctx.kernel_service.qr_code(actor, input.input).await?;

    let res = model::QrCode {
        base64_jpeg_qr_code,
    };
    Ok(api::Response::ok(res))
}
