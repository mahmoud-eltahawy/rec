use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize,Deserialize,FromRow)]
pub struct SparePart<T: ToString>{
  pub id         : T,
  pub name       : String,
}

impl SparePart::<Uuid>{
  pub fn string_to_client(self) -> SparePart<String>{
    SparePart {
      id: self.id.to_string(),
      name: self.name
    }
  }
}
