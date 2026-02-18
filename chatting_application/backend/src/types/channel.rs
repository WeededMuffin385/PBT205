use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Channel {
    pub name: String,
    pub channel_id: Uuid,
}