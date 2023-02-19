use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Name{
  pub id   : String,
  pub name : String
}
