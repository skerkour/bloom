use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn verify_email(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::VerifyEmail>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::VerifyEmailInput {
        code: input.code,
        pending_email_id: input.pending_email_id,
    };
    ctx.kernel_service.verify_email(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
