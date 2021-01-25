use crate::{
    api::kernel::model::{input, SignedUploadUrl},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn signed_upload_url(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::SignedUploadUrl>,
    actor: Actor,
) -> Result<api::Response<SignedUploadUrl>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::GetSignedUploadUrlInput {
        namespace_id: input.namespace_id,
        filesize: input.filesize,
    };
    let signed_url = ctx.kernel_service.get_signed_upload_url(actor, service_input).await?;

    Ok(api::Response::ok(signed_url.into()))
}
