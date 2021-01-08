use kernel::Actor;
use std::sync::Arc;
use stdx::uuid;

#[derive(Clone)]
pub struct ServerContext {
    pub kernel_service: Arc<kernel::Service>,
    pub files_service: Arc<files::Service>,
}

#[derive(Clone)]
pub struct RequestContext {
    pub server_ctx: Arc<ServerContext>,
    pub actor: Actor,
    pub request_id: Option<uuid::Uuid>,
}
