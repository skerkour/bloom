use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use kernel::http::api;
use std::sync::Arc;
use web::Json;

pub async fn trash(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::Trash>,
) -> Result<api::Response<File>, kernel::Error> {
    let input = input.into_inner();
    let file = ctx.files_service.find_trash(None, input.namespace).await?;

    Ok(api::Response::ok(file.into()))
}
