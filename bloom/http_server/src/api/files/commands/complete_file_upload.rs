use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use files::service;
use kernel::http::api;
use std::sync::Arc;
use web::Json;

pub async fn complete_file_upload(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CompleteFileUpload>,
) -> Result<api::Response<File>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CompleteFileUpload {
        upload_id: input.upload_id,
        parent_id: input.parent_id,
        name: input.name,
        mime_type: input.mime_type,
    };
    let file = ctx.files_service.complete_file_upload(None, service_input).await?;

    Ok(api::Response::ok(file.into()))
}
