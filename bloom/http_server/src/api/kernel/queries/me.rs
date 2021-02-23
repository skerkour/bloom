use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn me(ctx: web::Data<Arc<ServerContext>>, actor: Actor) -> Result<api::Response<model::Me>, kernel::Error> {
    let me = ctx.kernel_service.me(actor).await?;

    Ok(api::Response::ok(model::convert_me(&ctx.kernel_service, me)))
}
