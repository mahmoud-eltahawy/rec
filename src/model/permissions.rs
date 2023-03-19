 use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(PartialEq,Serialize,Deserialize,Debug)]
pub enum PermissionsNames {
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
pub struct Permissions {
  pub id                                                    :  Uuid,
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

impl Permissions {
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

#[derive(Serialize,Deserialize)]
pub struct ClientPermissions {
  pub id                                                    :  String,
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

impl ClientPermissions {
  pub fn list(&self) -> (Vec<PermissionsNames>,Vec<PermissionsNames>){
    let mut allowed   = Vec::new();
    let mut forbidden = Vec::new();
    if self.write_department_problem {
      allowed.push(PermissionsNames::WriteDepartmentProblem);
    } else {
      forbidden.push(PermissionsNames::WriteDepartmentProblem)
    }
    if self.read_department_problems {
      allowed.push(PermissionsNames::ReadDepartmentProblems);
    } else {
      forbidden.push(PermissionsNames::ReadDepartmentProblems)
    }
    if self.modify_department_problems {
      allowed.push(PermissionsNames::ModifyDepartmentProblems);
    } else {
      forbidden.push(PermissionsNames::ModifyDepartmentProblems)
    }
    if self.define_problem {
      allowed.push(PermissionsNames::DefineProblem);
    } else {
      forbidden.push(PermissionsNames::DefineProblem)
    }
    if self.access_history_department_problems {
      allowed.push(PermissionsNames::AccessHistoryDepartmentProblems);
    } else {
      forbidden.push(PermissionsNames::AccessHistoryDepartmentProblems)
    }
    if self.access_history_all_departments_problems {
      allowed.push(PermissionsNames::AccessHistoryAllDepartmentsProblems);
    } else {
      forbidden.push(PermissionsNames::AccessHistoryAllDepartmentsProblems)
    }
    if self.access_history_department_department_problems {
      allowed.push(PermissionsNames::AccessHistoryDepartmentDepartmentProblems)
    } else {
      forbidden.push(PermissionsNames::AccessHistoryDepartmentDepartmentProblems)
    }
    if self.access_history_all_departments_department_problems {
      allowed.push(PermissionsNames::AccessHistoryAllDepartmentsDepartmentProblems)
    } else {
      forbidden.push(PermissionsNames::AccessHistoryAllDepartmentsDepartmentProblems)
    }
    if self.access_history_machines {
      allowed.push(PermissionsNames::AccessHistoryMachines)
    } else {
      forbidden.push(PermissionsNames::AccessHistoryMachines)
    }
    if self.access_history_spare_parts {
      allowed.push(PermissionsNames::AccessHistorySpareParts)
    } else {
      forbidden.push(PermissionsNames::AccessHistorySpareParts)
    }
    if self.access_history_employees {
      allowed.push(PermissionsNames::AccessHistoryEmployees)
    } else {
      forbidden.push(PermissionsNames::AccessHistoryEmployees)
    }
    (allowed,forbidden)
  }

  pub fn from_list(id : String, allowed : Vec<PermissionsNames>) -> Self {
    let write_department_problem                           = allowed
      .contains(&PermissionsNames::WriteDepartmentProblem);
    let read_department_problems                           = allowed
      .contains(&PermissionsNames::ReadDepartmentProblems);
    let modify_department_problems                         = allowed
      .contains(&PermissionsNames::ModifyDepartmentProblems);
    let define_problem                                     = allowed
      .contains(&PermissionsNames::DefineProblem);
    let access_history_department_problems                 = allowed
      .contains(&PermissionsNames::AccessHistoryDepartmentProblems);
    let access_history_all_departments_problems            = allowed
      .contains(&PermissionsNames::AccessHistoryAllDepartmentsProblems);
    let access_history_department_department_problems      = allowed
      .contains(&PermissionsNames::AccessHistoryDepartmentDepartmentProblems);
    let access_history_all_departments_department_problems = allowed
      .contains(&PermissionsNames::AccessHistoryAllDepartmentsDepartmentProblems);
    let access_history_machines                            = allowed
      .contains(&PermissionsNames::AccessHistoryMachines);
    let access_history_spare_parts                         = allowed
      .contains(&PermissionsNames::AccessHistorySpareParts);
    let access_history_employees                           = allowed
      .contains(&PermissionsNames::AccessHistoryEmployees);
    return ClientPermissions {
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
