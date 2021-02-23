use crate::{
    api::kernel::model::{self, input},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn complete_sign_in(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CompleteSignIn>,
    actor: Actor,
) -> Result<api::Response<model::SignedIn>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CompleteSignInInput {
        pending_session_id: input.pending_session_id,
        code: input.code,
    };
    let res = ctx.kernel_service.complete_sign_in(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_signed_in(&ctx.kernel_service, res)))
}
