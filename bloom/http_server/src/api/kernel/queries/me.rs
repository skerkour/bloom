use crate::{api::kernel::model::Me, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn me(ctx: web::Data<Arc<ServerContext>>, actor: Actor) -> Result<api::Response<Me>, kernel::Error> {
    let me = ctx.kernel_service.me(actor).await?;

    Ok(api::Response::ok(me.into()))
}
