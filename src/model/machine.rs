use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{TableResponse, Wrapable};

#[derive(Serialize, Deserialize)]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
}

impl Wrapable for Machine {
    fn wrap(self) -> TableResponse {
        TableResponse::Machine(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateMachine {
    UpdateName(Uuid, String),
}
