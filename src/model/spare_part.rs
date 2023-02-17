use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Serialize,Deserialize,FromRow)]
pub struct SparePart{
    pub id         : Uuid,
    pub name       : String,
}


#[derive(Serialize,Deserialize,FromRow)]
pub struct ClientSparePart{
    pub id         : String,
    pub name       : String,
}

impl ClientSparePart{
    pub fn new(part : SparePart) -> Self{
        ClientSparePart {
            id: part.id.to_string(),
            name: part.name
        }
    }
}
