use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{TableResponse, Wrapable};

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Department {
    pub id: Uuid,
    pub boss_id: Option<Uuid>,
    pub name: String,
}

impl Wrapable for Department {
    fn wrap(self) -> TableResponse {
        TableResponse::Department(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateDepartment {
    SetBoss(Uuid, Uuid),
    ChangeBoss(Uuid),
    UpdateName(Uuid, String),
}
