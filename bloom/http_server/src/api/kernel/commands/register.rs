use super::super::model::{input, RegistrationStarted};
use crate::ServerContext;
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn register(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::Register>,
    actor: Actor,
) -> Result<api::Response<RegistrationStarted>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::RegisterInput {
        email: input.email,
        username: input.username,
    };
    let pending_user = ctx.kernel_service.register(actor, service_input).await?;

    Ok(api::Response::ok(RegistrationStarted {
        pending_user_id: pending_user.id.into(),
    }))
}
