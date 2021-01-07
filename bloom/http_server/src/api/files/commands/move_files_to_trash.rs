use crate::{
    api::{files::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use kernel::http::api;
use std::sync::Arc;
use web::Json;

pub async fn move_files_to_trash(
    _ctx: web::Data<Arc<ServerContext>>,
    _input: Json<input::MoveFilesToTrash>,
) -> Result<api::Response<Success>, kernel::Error> {
    unimplemented!(); // TODO
}
