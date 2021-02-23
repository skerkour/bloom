use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn accept_group_invitation(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::AcceptGroupInvitation>,
    actor: Actor,
) -> Result<api::Response<model::Group>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::AcceptGroupInvitationInput {
        invitation_id: input.invitation_id,
    };
    let group = ctx.kernel_service.accept_group_invitation(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_group(
        &ctx.kernel_service,
        group,
        true,
    )))
}
