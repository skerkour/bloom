use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn import_contacts(
    _ctx: web::Data<Arc<ServerContext>>,
    _input: Json<input::ImportContacts>,
    _actor: Actor,
) -> Result<api::Response<Vec<model::Contact>>, kernel::Error> {
    todo!();
}
