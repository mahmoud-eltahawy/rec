use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Clone,Copy,Debug)]
pub enum Table{
    Employee                = 1,
    Machine                 = 2,
    Department              = 3,
    SparePart               = 4,
    Problem                 = 5,
    Shift                   = 6,
    ShiftProblem            = 7,
    ShiftProblemProblem     = 8,
    ShiftProblemSparePart   = 9,
    ShiftProblemNote        = 10,
    ShiftNote               = 11,
    DepartmentShift         = 12,
    Undefined               = 0,
}

impl From<i16> for Table{
    fn from(value: i16) -> Self{
        match value {
            1  => Table::Employee,
            2  => Table::Machine,
            3  => Table::Department,
            4  => Table::SparePart,
            5  => Table::Problem,
            6  => Table::Shift,
            7  => Table::ShiftProblem,
            8  => Table::ShiftProblemProblem,
            9  => Table::ShiftProblemSparePart,
            10 => Table::ShiftProblemNote,
            11 => Table::ShiftNote,
            12 => Table::DepartmentShift,
            _ => Table::Undefined
        }
    }
}
#[derive(Serialize,Deserialize,Clone,Copy,Debug)]
pub enum Cud {
    Create = 1,
    Update = 2,
    Delete = 3,
    Undefined = 0
}

impl From<i16> for Cud{
    fn from(value: i16) -> Self{
        match value {
            1  => Cud::Create,
            2  => Cud::Update,
            3  => Cud::Delete,
            _ => Cud::Undefined
        }
    }
}

pub struct DbCudVersion{
    pub version_number      : i64,
    pub target_id           : Uuid,
    pub other_target_id     : Option<Uuid>,
    pub target_table        : i16,
    pub cud                 : i16,
}

#[derive(Serialize,Deserialize,Clone,Copy,Debug)]
pub struct CudVersion{
    pub version_number      : u64,
    pub target_id           : Uuid,
    pub other_target_id     : Option<Uuid>,
    pub target_table        : Table,
    pub cud                 : Cud,
}

impl CudVersion{
    pub fn get(v : DbCudVersion) -> Self{
        let DbCudVersion{cud,other_target_id,target_id,target_table,version_number} = v;
        let target_table : Table = target_table.into();
        let cud          : Cud = cud.into();
        CudVersion {
            version_number : version_number as u64,
            target_id,
            other_target_id,
            target_table,
            cud
        }
    }
}
