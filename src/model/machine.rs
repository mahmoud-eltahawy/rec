use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Serialize,Deserialize,FromRow)]
pub struct Machine{
    pub id          : Uuid,
    pub name        : String
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct ClientMachine{
    pub id          : String,
    pub name        : String
}

impl ClientMachine {
    pub fn new(machine : Machine) -> Self{
        ClientMachine {
            id: machine.id.to_string(),
            name: machine.name
        }
    }
}
