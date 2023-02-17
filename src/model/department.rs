use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Serialize,Deserialize,Clone,FromRow,Debug)]
pub struct Department{
   pub id            : Uuid,
   pub boss_id       : Option<Uuid>,
   pub department_id : Option<Uuid>,
   pub name          : String,
}

#[derive(Serialize,Deserialize,Clone,FromRow,Debug)]
pub struct ClientDepartment{
   pub id            : String,
   pub boss_id       : Option<String>,
   pub department_id : Option<String>,
   pub name          : String,
}

impl ClientDepartment {
   pub fn new(dep : Department) -> Self{
      let Department{id,boss_id,department_id,name} = dep;
      let id = id.to_string();
      let boss_id = match boss_id {
         Some(uuid) => Some(uuid.to_string()),
         None       => None
      };
      let department_id = match department_id {
         Some(uuid) => Some(uuid.to_string()),
         None       => None
      };
      ClientDepartment { id, boss_id, department_id, name }
   }
}
