use std::sync::Arc;
use stdx::uuid;
use stdx::uuid::Uuid;

#[derive(Clone)]
pub struct ServerContext {
    pub kernel_service: Arc<kernel::Service>,
}

#[derive(Clone)]
pub struct RequestContext {
    pub server_ctx: Arc<ServerContext>,
    pub actor: Actor,
    pub request_id: Option<uuid::Uuid>,
}

#[derive(Clone)]
pub enum Actor {
    User(kernel::entities::User),
    Anonymous(Uuid),
    None,
}
