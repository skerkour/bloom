use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn group_with_members_and_invitations(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::GetGroup>,
    actor: Actor,
) -> Result<api::Response<model::GroupWithMembersAndInvitations>, kernel::Error> {
    let input = input.into_inner();

    let group = ctx
        .kernel_service
        .find_group_members_and_invitations(actor, input.path)
        .await?;

    Ok(api::Response::ok(model::convert_group_with_members_and_invitations(
        &ctx.kernel_service,
        group,
    )))
}
