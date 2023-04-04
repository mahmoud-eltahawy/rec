use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct Problem<T: ToString>{
    pub id                      : T,
    pub writer_id               : T,
    pub department_id           : T,
    pub title                   : String,
    pub description             : String
}

impl Problem::<Uuid> {
    pub fn string_to_client(self) -> Problem<String>{
        let Problem{id,writer_id,department_id,title,description} = self;
        let id = id.to_string();
        let writer_id = writer_id.to_string();
        let department_id = department_id.to_string();
        Problem { id, writer_id, department_id , title, description }
    }
}
