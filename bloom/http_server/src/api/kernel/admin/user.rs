use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn user(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::AdminGetUser>,
    actor: Actor,
) -> Result<api::Response<model::User>, kernel::Error> {
    let input = input.into_inner();
    let user = ctx.kernel_service.admin_find_user(actor, input.user_id).await?;

    Ok(api::Response::ok(model::convert_user(&ctx.kernel_service, user, true)))
}
