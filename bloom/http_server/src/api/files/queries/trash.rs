use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn trash(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::Trash>,
    actor: Actor,
) -> Result<api::Response<Vec<File>>, kernel::Error> {
    let input = input.into_inner();
    let file = ctx.files_service.find_trash(actor, input.namespace_id).await?;

    Ok(api::Response::ok(file.into_iter().map(Into::into).collect()))
}
