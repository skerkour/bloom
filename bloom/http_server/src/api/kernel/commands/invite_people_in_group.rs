use crate::{
    api::kernel::model::{input, GroupInvitation},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service};
use std::sync::Arc;
use web::Json;

pub async fn invite_people_in_group(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::InvitePeopleInGroup>,
) -> Result<api::Response<Vec<GroupInvitation>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::InvitePeopleInGroupInput {
        group_id: input.group_id,
        usernames: input.usernames,
    };
    let invitations = ctx.kernel_service.invite_people_in_group(None, service_input).await?;

    Ok(api::Response::ok(invitations.into_iter().map(Into::into).collect()))
}
