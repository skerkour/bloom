use crate::{
    api::files::model::{input, File},
    ServerContext,
};
use actix_web::web;
use files::service;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn create_folder(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CreateFolder>,
    actor: Actor,
) -> Result<api::Response<File>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CreateFolderInput {
        parent_id: input.parent_id,
        name: input.name,
    };
    let file = ctx.files_service.create_folder(actor, service_input).await?;

    Ok(api::Response::ok(file.into()))
}
