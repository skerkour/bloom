use crate::{
    api::{kernel::model::Success, newsletter::model::input},
    ServerContext,
};
use actix_web::web;
use inbox::service::RemoveContactFromListInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn remove_contact_from_list(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::RemoveContactFromList>,
    actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    let input = input.into_inner();
    let service_input = RemoveContactFromListInput {
        list_id: input.list_id,
        contact_id: input.contact_id,
    };
    ctx.inbox_service.remove_contact_from_list(actor, service_input).await?;

    Ok(api::Response::ok(true.into()))
}
