use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use kernel::http::api;
use std::sync::Arc;
use web::Json;

pub async fn complete_file_upload(
    _ctx: web::Data<Arc<ServerContext>>,
    _input: Json<input::CompleteFileUpload>,
) -> Result<api::Response<File>, kernel::Error> {
    unimplemented!();
}
