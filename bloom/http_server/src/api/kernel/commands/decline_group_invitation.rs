use crate::{
    api::kernel::model::{input, Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn decline_group_invitation(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::DeclineGroupInvitation>,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::DeclineGroupInvitationInput {
        invitation_id: input.invitation_id,
    };
    ctx.kernel_service.decline_group_invitation(None, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
