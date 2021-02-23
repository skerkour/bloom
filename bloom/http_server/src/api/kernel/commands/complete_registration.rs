use crate::{
    api::kernel::model::{self, input},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn complete_registration(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CompleteRegistration>,
    actor: Actor,
) -> Result<api::Response<model::SignedIn>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CompleteRegistrationInput {
        pending_user_id: input.pending_user_id,
        code: input.code,
    };
    let res = ctx.kernel_service.complete_registration(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_registered(&ctx.kernel_service, res)))
}
