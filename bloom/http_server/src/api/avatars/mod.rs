use crate::ServerContext;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn avatars(ctx: web::Data<Arc<ServerContext>>, web::Path(avatar_id): web::Path<String>) -> HttpResponse {
    let avatar = match ctx.kernel_service.serve_avatar(avatar_id).await {
        Ok(data) => data,
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", &err)),
    };

    HttpResponse::Ok()
        .content_type(kernel::consts::AVATAR_CONTENT_TYPE)
        .body(avatar)
}
