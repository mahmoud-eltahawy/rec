 use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(PartialEq,Serialize,Deserialize,Debug)]
pub enum PermissionName {
  WriteDepartmentProblem,
  ReadDepartmentProblems,
  ModifyDepartmentProblems,
  DefineProblem,
  AccessHistoryDepartmentProblems,
  AccessHistoryAllDepartmentsProblems,
  AccessHistoryDepartmentDepartmentProblems,
  AccessHistoryAllDepartmentsDepartmentProblems,
  AccessHistoryMachines,
  AccessHistorySpareParts,
  AccessHistoryEmployees,
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Permissions<T:ToString>{
  pub id                                                    :  T,
  pub write_department_problem                              :  bool,
  pub read_department_problems                              :  bool,
  pub modify_department_problems                            :  bool,
  pub define_problem                                        :  bool,
  pub access_history_department_problems                    :  bool,
  pub access_history_all_departments_problems               :  bool,
  pub access_history_department_department_problems         :  bool,
  pub access_history_all_departments_department_problems    :  bool,
  pub access_history_machines                               :  bool,
  pub access_history_spare_parts                            :  bool,
  pub access_history_employees                              :  bool,
}

impl Permissions::<Uuid> {
  pub fn string_to_client(self) -> Permissions<String>{
    let Permissions{
      id,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      define_problem,
      modify_department_problems,
      read_department_problems,
      write_department_problem,
    } = self;
    Permissions {
      id: id.to_string(),
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      define_problem,
      modify_department_problems,
      read_department_problems,
      write_department_problem,
    }
  }
  pub fn default(id : Uuid) -> Self{
      Permissions{
        id,
        access_history_all_departments_department_problems  : false,
        access_history_all_departments_problems             : false,
        access_history_department_department_problems       : false,
        access_history_department_problems                  : false,
        access_history_employees                            : false,
        access_history_machines                             : false,
        access_history_spare_parts                          : false,
        define_problem                                      : false,
        modify_department_problems                          : false,
        read_department_problems                            : false,
        write_department_problem                            : false,
    }
  }
}

impl Permissions::<String> {
  pub fn list(&self) -> (Vec<PermissionName>,Vec<PermissionName>){
    let mut allowed   = Vec::new();
    let mut forbidden = Vec::new();
    if self.write_department_problem {
      allowed.push(PermissionName::WriteDepartmentProblem);
    } else {
      forbidden.push(PermissionName::WriteDepartmentProblem)
    }
    if self.read_department_problems {
      allowed.push(PermissionName::ReadDepartmentProblems);
    } else {
      forbidden.push(PermissionName::ReadDepartmentProblems)
    }
    if self.modify_department_problems {
      allowed.push(PermissionName::ModifyDepartmentProblems);
    } else {
      forbidden.push(PermissionName::ModifyDepartmentProblems)
    }
    if self.define_problem {
      allowed.push(PermissionName::DefineProblem);
    } else {
      forbidden.push(PermissionName::DefineProblem)
    }
    if self.access_history_department_problems {
      allowed.push(PermissionName::AccessHistoryDepartmentProblems);
    } else {
      forbidden.push(PermissionName::AccessHistoryDepartmentProblems)
    }
    if self.access_history_all_departments_problems {
      allowed.push(PermissionName::AccessHistoryAllDepartmentsProblems);
    } else {
      forbidden.push(PermissionName::AccessHistoryAllDepartmentsProblems)
    }
    if self.access_history_department_department_problems {
      allowed.push(PermissionName::AccessHistoryDepartmentDepartmentProblems)
    } else {
      forbidden.push(PermissionName::AccessHistoryDepartmentDepartmentProblems)
    }
    if self.access_history_all_departments_department_problems {
      allowed.push(PermissionName::AccessHistoryAllDepartmentsDepartmentProblems)
    } else {
      forbidden.push(PermissionName::AccessHistoryAllDepartmentsDepartmentProblems)
    }
    if self.access_history_machines {
      allowed.push(PermissionName::AccessHistoryMachines)
    } else {
      forbidden.push(PermissionName::AccessHistoryMachines)
    }
    if self.access_history_spare_parts {
      allowed.push(PermissionName::AccessHistorySpareParts)
    } else {
      forbidden.push(PermissionName::AccessHistorySpareParts)
    }
    if self.access_history_employees {
      allowed.push(PermissionName::AccessHistoryEmployees)
    } else {
      forbidden.push(PermissionName::AccessHistoryEmployees)
    }
    (allowed,forbidden)
  }

  pub fn from_list(id : String, allowed : Vec<PermissionName>) -> Self {
    let write_department_problem                           = allowed
      .contains(&PermissionName::WriteDepartmentProblem);
    let read_department_problems                           = allowed
      .contains(&PermissionName::ReadDepartmentProblems);
    let modify_department_problems                         = allowed
      .contains(&PermissionName::ModifyDepartmentProblems);
    let define_problem                                     = allowed
      .contains(&PermissionName::DefineProblem);
    let access_history_department_problems                 = allowed
      .contains(&PermissionName::AccessHistoryDepartmentProblems);
    let access_history_all_departments_problems            = allowed
      .contains(&PermissionName::AccessHistoryAllDepartmentsProblems);
    let access_history_department_department_problems      = allowed
      .contains(&PermissionName::AccessHistoryDepartmentDepartmentProblems);
    let access_history_all_departments_department_problems = allowed
      .contains(&PermissionName::AccessHistoryAllDepartmentsDepartmentProblems);
    let access_history_machines                            = allowed
      .contains(&PermissionName::AccessHistoryMachines);
    let access_history_spare_parts                         = allowed
      .contains(&PermissionName::AccessHistorySpareParts);
    let access_history_employees                           = allowed
      .contains(&PermissionName::AccessHistoryEmployees);
    return Permissions {
      id,
      write_department_problem,
      read_department_problems,
      modify_department_problems,
      define_problem,
      access_history_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_all_departments_department_problems,
      access_history_machines,
      access_history_spare_parts,
      access_history_employees
    };
  }
}
