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
}
