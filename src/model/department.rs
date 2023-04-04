use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Serialize,Deserialize,Clone,FromRow,Debug)]
pub struct Department<T: ToString>{
   pub id            : T,
   pub boss_id       : Option<T>,
   pub department_id : Option<T>,
   pub name          : String,
}

impl Department::<Uuid> {
   pub fn string_to_client(self) -> Department<String>{
      let Department{id,boss_id,department_id,name} = self;
      let id = id.to_string();
      let boss_id = match boss_id {
         Some(uuid) => Some(uuid.to_string()),
         None       => None
      };
      let department_id = match department_id {
         Some(uuid) => Some(uuid.to_string()),
         None       => None
      };
      Department { id, boss_id, department_id, name }
   }
}
