use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct Machine{
    pub id          : Uuid,
    pub name        : String
}

#[derive(Serialize,Deserialize,Clone)]
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
