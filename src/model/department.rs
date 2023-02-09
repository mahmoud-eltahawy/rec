use uuid::Uuid;


pub struct DbDepartment{
   pub id            : Uuid,
   pub boss_id       : Option<Uuid>,
   pub department_id : Option<Uuid>,
   pub name          : String,
}
