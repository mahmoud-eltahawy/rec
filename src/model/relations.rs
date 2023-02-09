use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct ShiftProblemProblem{
    pub shift_problem_id    : Uuid,
    pub problem_id          : Uuid
}

#[derive(Serialize,Deserialize)]
pub struct ShiftProblemSparePart{
    pub shift_problem_id    : Uuid,
    pub spare_part_id       : Uuid
}
