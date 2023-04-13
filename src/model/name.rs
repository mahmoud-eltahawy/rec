use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Name {
    pub id: Uuid,
    pub name: String,
}
