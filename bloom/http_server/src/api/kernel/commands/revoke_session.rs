use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn revoke_session(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::RevokeSession>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::RevokeSessionInput {
        session_id: input.session_id,
    };
    ctx.kernel_service.revoke_session(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
