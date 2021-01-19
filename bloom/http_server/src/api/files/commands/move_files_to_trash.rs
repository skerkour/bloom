use crate::{
    api::{files::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use files::service;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn move_files_to_trash(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::MoveFilesToTrash>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::MoveFilesToTrashInput {
        files: input.files,
    };
    ctx.files_service.move_files_to_trash(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
