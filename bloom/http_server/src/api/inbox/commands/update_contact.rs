use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use inbox::service::UpdateContactInput;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn update_contact(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::UpdateContact>,
    actor: Actor,
) -> Result<api::Response<model::Contact>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UpdateContactInput {
        contact_id: input.contact_id,
        name: input.name,
        birthday: input.birthday,
        email: input.email,
        pgp_key: input.pgp_key,
        phone: input.phone,
        address: input.address,
        website: input.website,
        twitter: input.twitter,
        instagram: input.instagram,
        facebook: input.facebook,
        linkedin: input.linkedin,
        skype: input.skype,
        telegram: input.telegram,
        bloom: input.bloom,
        notes: input.notes,
        plan: input.plan,
        user_id: input.user_id,
    };
    let contact = ctx.inbox_service.update_contact(actor, service_input).await?;

    Ok(api::Response::ok(model::convert_contact(&ctx.kernel_service, contact)))
}
