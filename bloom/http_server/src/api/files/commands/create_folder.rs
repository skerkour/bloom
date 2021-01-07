use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use kernel::http::api;
use std::sync::Arc;
use web::Json;

pub async fn create_folder(
    _ctx: web::Data<Arc<ServerContext>>,
    _input: Json<input::CreateFolder>,
) -> Result<api::Response<File>, kernel::Error> {
    unimplemented!();
}
