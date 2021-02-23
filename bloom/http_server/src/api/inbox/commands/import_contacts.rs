use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::ImportContactsInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn import_contacts(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::ImportContacts>,
    actor: Actor,
) -> Result<api::Response<Vec<model::Contact>>, kernel::Error> {
    let input = input.into_inner();
    let service_input = ImportContactsInput {
        namespace_id: input.namespace_id,
        list_id: input.list_id,
        contacts_csv: input.contacts_csv,
    };
    let contacts = ctx.inbox_service.import_contacts(actor, service_input).await?;

    Ok(api::Response::ok(
        contacts
            .into_iter()
            .map(|contact| model::convert_contact(&ctx.kernel_service, contact))
            .collect(),
    ))
}
