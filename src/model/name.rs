use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize,Deserialize,FromRow,Clone,Debug)]
pub struct Name{
  pub id   : String,
  pub name : String
}
