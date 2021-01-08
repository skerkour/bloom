use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use files::service;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn rename_file(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::RenameFile>,
    actor: Actor,
) -> Result<api::Response<File>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::RenameFileInput {
        file_id: input.file_id,
        name: input.name,
    };
    let file = ctx.files_service.rename_file(actor, service_input).await?;

    Ok(api::Response::ok(file.into()))
}
