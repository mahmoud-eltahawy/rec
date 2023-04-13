use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{
    note::{Note, ShiftNote},
    TableResponse, Wrapable,
};

const FIRST_SHIFT: &str = "FIRST";
const SECOND_SHIFT: &str = "SECOND";
const THIRD_SHIFT: &str = "THIRD";

#[derive(Serialize, Deserialize, Clone)]
pub enum ShiftOrder {
    FIRST,
    SECOND,
    THIRD,
}

impl ShiftOrder {
    pub fn stringify(&self) -> String {
        match self {
            ShiftOrder::FIRST => FIRST_SHIFT.to_string(),
            ShiftOrder::SECOND => SECOND_SHIFT.to_string(),
            ShiftOrder::THIRD => THIRD_SHIFT.to_string(),
        }
    }
}

impl TryFrom<String> for ShiftOrder {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            FIRST_SHIFT => Ok(ShiftOrder::FIRST),
            SECOND_SHIFT => Ok(ShiftOrder::SECOND),
            THIRD_SHIFT => Ok(ShiftOrder::THIRD),
            _ => Err("undefined shift".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Shift {
    pub id: Uuid,
    pub shift_date: NaiveDate,
    pub shift_order: ShiftOrder,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DepartmentShift {
    pub id: Uuid,
    pub shift_id: Uuid,
    pub department_id: Uuid,
}

impl Wrapable for DepartmentShift {
    fn wrap(self) -> TableResponse {
        TableResponse::DepartmentShift(self)
    }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateDepartmentShift {
    SaveShiftEmployee(Uuid, Uuid),
    DeleteShiftEmployee(Uuid, Uuid),
    SaveNote(ShiftNote),
    DeleteNote(Uuid, Uuid),
    UpdateNote(Note),
}
