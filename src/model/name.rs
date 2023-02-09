use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::{
    employee::Employee,
    problem::Probelm,
    machine::Machine,
    spare_part::SparePart
};

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Name{
  pub id : Option<Uuid>,
  pub name : String
}

impl Name{
  pub fn build_employee(employee : &Employee) -> Name{
    Name {
      id: employee.id,
      name: format!("{} {} {}",
              employee.first_name,
              employee.middle_name,
              employee.last_name
      )
    }
  }

  pub fn build_problem(problem : &Probelm) -> Name{
    Name {
      id: Some(problem.id),
      name: problem.title.clone()
    }
  }

  pub fn build_machine(machine : &Machine) -> Name{
    Name {
      id: Some(machine.id),
      name: machine.name.clone()
    }
  }

  pub fn build_spare_part(spare_part : &SparePart) -> Name{
    Name {
      id: Some(spare_part.id),
      name: spare_part.name.clone()
    }
  }
}
