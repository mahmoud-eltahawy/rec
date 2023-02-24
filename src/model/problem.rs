use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct Probelm{
    pub id                      : Uuid,
    pub writer_id               : Uuid,
    pub department_id           : Uuid,
    pub title                   : String,
    pub description             : String
}

#[derive(Serialize,Deserialize)]
pub struct ClientProblem{
    pub id                      : String,
    pub writer_id               : String,
    pub department_id           : String,
    pub title                   : String,
    pub description             : String
}

impl ClientProblem {
    pub fn new(problem : Probelm) -> Self{
        let Probelm{id,writer_id,department_id,title,description} = problem;
        let id = id.to_string();
        let writer_id = writer_id.to_string();
        let department_id = department_id.to_string();
        ClientProblem { id, writer_id, department_id , title, description }
    }
}
