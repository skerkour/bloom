use crate::{
    api::{files::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use files::service;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn restore_files_from_trash(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::RestoreFilesFromTrash>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::RestoreFilesFromTrashInput {
        files: input.files,
    };
    ctx.files_service.restore_files_from_trash(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
