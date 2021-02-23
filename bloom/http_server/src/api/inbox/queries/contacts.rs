use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::FindContactsInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn contacts(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::GetContacts>,
    actor: Actor,
) -> Result<api::Response<Vec<model::Contact>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = FindContactsInput {
        namespace_id: input.namespace_id,
    };
    let contacts = ctx.inbox_service.find_contacts(actor, service_input).await?;

    Ok(api::Response::ok(
        contacts
            .into_iter()
            .map(|c| model::convert_contact(&ctx.kernel_service, c))
            .collect(),
    ))
}
