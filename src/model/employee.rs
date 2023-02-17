use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Serialize,Deserialize,Clone,FromRow,Debug)]
pub struct Employee{
    pub id              : Uuid,
    pub department_id   : Uuid,
    pub position        : String,
    pub first_name      : String,
    pub middle_name     : String,
    pub last_name       : String,
    pub card_id         : i16,
    pub password        : String
}

#[derive(Serialize,Deserialize,Clone,FromRow,Debug)]
pub struct ClientEmployee{
    pub id              : String,
    pub department_id   : String,
    pub position        : String,
    pub first_name      : String,
    pub middle_name     : String,
    pub last_name       : String,
    pub card_id         : i16,
    pub password        : String
}

impl ClientEmployee{
  pub fn new(employee : Employee) -> Self{
    let Employee{card_id,department_id,first_name,id,last_name,middle_name,password,position} = employee;
    ClientEmployee {
        id: id.to_string(),
        department_id: department_id.to_string(),
        position,
        first_name,
        middle_name,
        last_name,
        card_id,
        password
    }
  }
}

#[derive(Serialize,Deserialize)]
pub struct Cred{
  pub card_id : i16,
  pub password: String
}
