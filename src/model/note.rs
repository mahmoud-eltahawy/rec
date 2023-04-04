use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct DbNote<T:ToString>
{
  pub id                : T,
  pub shift_id          : Option<T>,
  pub shift_problem_id  : Option<T>,
  pub content           : String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Note<T: ToString>{
  pub id      : T,
  pub content : String
}

impl DbNote<Uuid> {
    pub fn string_to_client(self) -> DbNote<String>{
      DbNote {
        id: self.id.to_string(),
        shift_id: self.shift_id.map(|id| id.to_string()),
        shift_problem_id: self.shift_problem_id.map(|id| id.to_string()),
        content: self.content
      }
    }
}

impl Note<Uuid> {
    pub fn string_to_client(self) -> Note<String>{
      Note {
        id: self.id.to_string(),
        content: self.content
      }
    }
}
