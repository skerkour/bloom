use crate::{
    api::kernel::model::{input, SignedIn},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn complete_registration(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CompleteRegistration>,
) -> Result<api::Response<SignedIn>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CompleteRegistrationInput {
        pending_user_id: input.pending_user_id,
        code: input.code,
    };
    let res = ctx.kernel_service.complete_registration(None, service_input).await?;

    Ok(api::Response::ok(res.into()))
}
