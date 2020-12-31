use crate::{
    api::kernel::model::{input, SignedIn},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn complete_sign_in(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CompleteSignIn>,
) -> Result<api::Response<SignedIn>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CompleteSignInInput {
        pending_session_id: input.pending_session_id,
        code: input.code,
    };
    let res = ctx.kernel_service.complete_sign_in(None, service_input).await?;

    Ok(api::Response::ok(res.into()))
}
