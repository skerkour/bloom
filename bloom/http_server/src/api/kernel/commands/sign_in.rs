use super::super::model::{input, SignInStarted};
use crate::ServerContext;
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn sign_in(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SignIn>,
) -> Result<api::Response<SignInStarted>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::SignInInput {
        email_or_username: input.email_or_username,
    };
    let pending_session = ctx.kernel_service.sign_in(None, service_input).await?;

    Ok(api::Response::ok(pending_session.into()))
}
