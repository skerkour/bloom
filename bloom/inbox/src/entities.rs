use stdx::{sqlx, uuid::Uuid};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
}
