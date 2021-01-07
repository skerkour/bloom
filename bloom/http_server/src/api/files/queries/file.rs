use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use files::service;
use kernel::http::api;
use std::sync::Arc;
use web::Json;

pub async fn file(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::File>,
) -> Result<api::Response<File>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::FindFileInput {
        file_id: input.file_id,
        namespace: input.namespace,
    };
    let file = ctx.files_service.find_file(None, service_input).await?;

    Ok(api::Response::ok(file.into()))
}
