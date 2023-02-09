use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct DbNote{
  pub id                : Uuid,
  pub shift_id          : Option<Uuid>,
  pub shift_problem_id  : Option<Uuid>,
  pub content           : String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Note{
  pub id   : Uuid,
  pub content : String
}
