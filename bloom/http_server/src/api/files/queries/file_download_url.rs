use crate::{
    api::files::model::{input, FileDownloadUrl},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn file_download_url(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::FileDownloadUrl>,
    actor: Actor,
) -> Result<api::Response<FileDownloadUrl>, kernel::Error> {
    let input = input.into_inner();
    let url = ctx.files_service.get_file_download_url(actor, input.file_id).await?;

    Ok(api::Response::ok(FileDownloadUrl {
        url,
    }))
}
