use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Serialize,Deserialize,FromRow)]
pub struct Probelm{
    pub id                      : Uuid,
    pub writer_id               : Uuid,
    pub department_id           : Uuid,
    pub title                   : String,
    pub description             : String
}
