use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct Machine<T: ToString>{
    pub id          : T,
    pub name        : String
}

impl Machine<Uuid> {
    pub fn string_to_client(self) -> Machine<String>{
        Machine {
            id: self.id.to_string(),
            name: self.name
        }
    }
}
