use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{permissions::PermissionName, TableResponse, Wrapable};

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Employee {
    pub id: Uuid,
    pub department_id: Uuid,
    pub position: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub card_id: i64,
    pub password: String,
}

impl Wrapable for Employee {
    fn wrap(self) -> TableResponse {
        TableResponse::Employee(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateEmployee {
    UpdatePassword(Uuid, String),
    UpdateDepartment(Uuid, Uuid),
    Up(Uuid),
    Down(Uuid),
    ForbidAllPermissions(Uuid),
    ForbidPermission(Uuid, PermissionName),
    AllowPermission(Uuid, PermissionName),
}
