use crate::{
    api::kernel::model::{input, User},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, service, Actor};
use std::sync::Arc;
use web::Json;

pub async fn update_my_profile(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UpdateMyProfile>,
    actor: Actor,
) -> Result<api::Response<User>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::UpdateMyProfileInput {
        username: input.username,
        email: input.email,
        name: input.name,
        description: input.description,
    };
    let me = ctx.kernel_service.update_my_profile(actor, service_input).await?;

    Ok(api::Response::ok(me.into()))
}
