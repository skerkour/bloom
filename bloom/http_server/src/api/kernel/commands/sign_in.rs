use super::super::model::{input, SignInStarted};
use crate::ServerContext;
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn sign_in(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SignIn>,
    actor: Actor,
) -> Result<api::Response<SignInStarted>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::SignInInput {
        email_or_username: input.email_or_username,
    };
    let pending_session = ctx.kernel_service.sign_in(actor, service_input).await?;

    Ok(api::Response::ok(pending_session.into()))
}
