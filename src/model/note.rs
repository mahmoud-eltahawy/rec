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
pub struct ClientDbNote{
  pub id                : String,
  pub shift_id          : Option<String>,
  pub shift_problem_id  : Option<String>,
  pub content           : String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Note{
  pub id      : Uuid,
  pub content : String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ClientNote{
  pub id      : String,
  pub content : String
}
