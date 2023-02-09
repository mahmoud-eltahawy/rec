use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Note{
  pub id   : Uuid,
  pub content : String
}
