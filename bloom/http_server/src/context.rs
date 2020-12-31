use std::sync::Arc;
use stdx::uuid;

#[derive(Clone)]
pub struct ServerContext {
    pub kernel_service: Arc<kernel::Service>,
}

#[derive(Clone)]
pub struct RequestContext {
    pub server_ctx: Arc<ServerContext>,
    pub actor: Option<kernel::entities::User>,
    pub request_id: Option<uuid::Uuid>,
}
