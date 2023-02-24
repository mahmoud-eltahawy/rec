use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::timer::ShiftOrder;

#[derive(Serialize,Deserialize,FromRow)]
pub struct DbShift{
    pub id            : Uuid,
    pub shift_date    : NaiveDate,
    pub shift_order   : i16,
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct DateOrder{
    pub date    : NaiveDate,
    pub order   : i16,
}


#[derive(Serialize,Deserialize,FromRow)]
pub struct Shift{
    pub id            : Uuid,
    pub shift_date    : NaiveDate,
    pub shift_order   : ShiftOrder,
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct DepartmentShift{
    pub id              : Uuid,
    pub shift_id        : Uuid,
    pub department_id   : Uuid,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ClientDepartmentShift{
    pub id              : String,
    pub shift_id        : String,
    pub department_id   : String,
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct ClientDbShift{
    pub id            : String,
    pub shift_date    : String,
    pub shift_order   : String,
}

impl Shift {
    pub fn new(s : DbShift) -> Option<Self>{
        let shift_order = match s.shift_order {
            1 => ShiftOrder::FIRST,
            2 => ShiftOrder::SECOND,
            3 => ShiftOrder::THIRD,
            _ => return None
        };
        Some(Shift {
            id: s.id,
            shift_date: s.shift_date,
            shift_order
        })
    }
    pub fn get(s : ClientDbShift) -> Option<Self>{
        let ClientDbShift{id,shift_order,shift_date} = s;
        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(_) => return None
        };
        let shift_date = match serde_json::from_str(&shift_date) {
            Ok(date) => date,
            Err(_)   => return None
        };
        let shift_order = match serde_json::from_str(&shift_order) {
            Ok(date) => date,
            Err(_)   => return None
        };
        Some(Shift {
            id,
            shift_date,
            shift_order
        })
    }
}

impl ClientDbShift {
    pub fn new(s : Shift) -> Self {
        let Shift { id, shift_date, shift_order } = s;
        let shift_order = serde_json::json!(shift_order).to_string();
        let shift_date  = serde_json::json!(shift_date).to_string();
        ClientDbShift {
            id: id.to_string(),
            shift_order,
            shift_date
        }
    }
}

impl ClientDepartmentShift {
    pub fn new(shift : DepartmentShift) -> Self {
        let DepartmentShift { id, shift_id, department_id } = shift;
        ClientDepartmentShift {
            id              : id.to_string(),
            shift_id        : shift_id.to_string(),
            department_id   : department_id.to_string()
        }
    }
}
