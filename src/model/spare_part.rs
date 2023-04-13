use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{TableResponse, Wrapable};

#[derive(Serialize, Deserialize, FromRow)]
pub struct SparePart {
    pub id: Uuid,
    pub name: String,
}

impl Wrapable for SparePart {
    fn wrap(self) -> TableResponse {
        TableResponse::SparePart(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateSparePart {
    UpdateName(Uuid, String),
}
