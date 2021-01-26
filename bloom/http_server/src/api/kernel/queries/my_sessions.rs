use crate::{api::kernel::model::Session, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn my_sessions(
    ctx: web::Data<Arc<ServerContext>>,
    actor: Actor,
) -> Result<api::Response<Vec<Session>>, kernel::Error> {
    let sessions = ctx.kernel_service.find_my_sessions(actor).await?;

    Ok(api::Response::ok(sessions.into_iter().map(Into::into).collect()))
}
