use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn markdown(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::Markdown>,
    actor: Actor,
) -> Result<api::Response<model::MarkdownHtml>, kernel::Error> {
    let input = input.into_inner();

    let html = ctx.kernel_service.markdown(actor, input.markdown).await?;

    Ok(api::Response::ok(model::MarkdownHtml {
        html,
    }))
}
