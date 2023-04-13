use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{TableResponse, Wrapable};

#[derive(Serialize, Deserialize)]
pub struct Problem {
    pub id: Uuid,
    pub department_id: Uuid,
    pub title: String,
    pub description: String,
}

impl Wrapable for Problem {
    fn wrap(self) -> TableResponse {
        TableResponse::Problem(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateProblem {
    UpdateTitle(Uuid, String),
    UpdateDescription(Uuid, String),
}
