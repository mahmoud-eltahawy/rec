use serde::{Serialize, Deserialize};

use super::{
    employee::ClientEmployee,
    problem::ClientProblem,
    machine::ClientMachine,
    spare_part::ClientSparePart
};

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Name{
  pub id : String,
  pub name : String
}

impl Name{
  pub fn build_employee(employee : ClientEmployee) -> Name{
    Name {
      id: employee.id,
      name: format!("{} {} {}",
              employee.first_name,
              employee.middle_name,
              employee.last_name
      )
    }
  }

  pub fn build_problem(problem : ClientProblem) -> Name{
    Name {
      id: problem.id,
      name: problem.title.clone()
    }
  }

  pub fn build_machine(machine : ClientMachine) -> Name{
    Name {
      id: machine.id,
      name: machine.name.clone()
    }
  }

  pub fn build_spare_part(spare_part : ClientSparePart) -> Name{
    Name {
      id: spare_part.id,
      name: spare_part.name.clone()
    }
  }
}
