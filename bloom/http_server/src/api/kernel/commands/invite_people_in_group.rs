use crate::{
    api::kernel::model::{self, input},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn invite_people_in_group(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::InvitePeopleInGroup>,
    actor: Actor,
) -> Result<api::Response<Vec<model::GroupInvitation>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::InvitePeopleInGroupInput {
        group_id: input.group_id,
        usernames: input.usernames,
    };
    let invitations = ctx.kernel_service.invite_people_in_group(actor, service_input).await?;

    Ok(api::Response::ok(
        invitations
            .into_iter()
            .map(|i| model::convert_group_invitation_with_details(&ctx.kernel_service, i))
            .collect(),
    ))
}
