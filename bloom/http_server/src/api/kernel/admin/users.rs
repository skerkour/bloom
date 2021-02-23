use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn users(
    ctx: web::Data<Arc<ServerContext>>,
    actor: Actor,
) -> Result<api::Response<Vec<model::User>>, kernel::Error> {
    let users = ctx.kernel_service.admin_find_users(actor).await?;

    Ok(api::Response::ok(
        users
            .into_iter()
            .map(|user| model::convert_user(&ctx.kernel_service, user, true))
            .collect(),
    ))
}
