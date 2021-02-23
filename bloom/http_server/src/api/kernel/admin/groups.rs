use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn groups(
    ctx: web::Data<Arc<ServerContext>>,
    actor: Actor,
) -> Result<api::Response<Vec<model::Group>>, kernel::Error> {
    let groups = ctx.kernel_service.admin_find_groups(actor).await?;

    Ok(api::Response::ok(
        groups
            .into_iter()
            .map(|group| model::convert_group(&ctx.kernel_service, group, true))
            .collect(),
    ))
}
