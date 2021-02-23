use crate::{
    api::kernel::model::{self, input},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn complete_two_fa_challenge(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CompleteTwoFaChallenge>,
    actor: Actor,
) -> Result<api::Response<model::SignedIn>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CompleteTwoFaChallengeInput {
        pending_session_id: input.pending_session_id,
        code: input.code,
    };
    let res = ctx
        .kernel_service
        .complete_two_fa_challenge(actor, service_input)
        .await?;

    Ok(api::Response::ok(model::convert_signed_in(&ctx.kernel_service, res)))
}
