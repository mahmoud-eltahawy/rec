use chrono::NaiveTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::note::{Note, ClientNote};

#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct MinimamlShiftProblem{
  pub id                : Uuid,
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
  pub problems_ids      : Vec<Uuid>,
  pub spare_parts_ids   : Option<Vec<Uuid>>,
  pub note              : Option<Note>
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct ShiftProblem{
  pub id                : Uuid,
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
}

impl MinimamlShiftProblem {
  pub fn new(problem : ProblemDetail) -> Self{
    let ProblemDetail{
      shift_id,
      writer_id,
      begin_time,
      end_time,
      machine_id,
      problems_ids,
      spare_parts_ids,
      maintainer_id,
      note
    } = problem;
    let note = match note {
      Some(content) =>Some(Note{
        id : Uuid::new_v4(),
        content
      }),
      None => None
    };
    MinimamlShiftProblem {
      id : Uuid::new_v4(),
      shift_id,
      writer_id,
      maintainer_id,
      machine_id,
      begin_time,
      end_time,
      problems_ids,
      spare_parts_ids,
      note
    }
  }
  pub fn destruct(self) -> (ShiftProblem,Vec<Uuid>,Option<Vec<Uuid>>,Option<Note>){
    let MinimamlShiftProblem{ id, shift_id, writer_id, maintainer_id,
            machine_id, begin_time, end_time, problems_ids, spare_parts_ids, note } = self;
    let shift_problem = ShiftProblem{id,machine_id,maintainer_id,shift_id,writer_id,begin_time,end_time};
    (shift_problem,problems_ids,spare_parts_ids,note)
  }
  pub fn construct(pieaces : (ShiftProblem,Vec<Uuid>,Option<Vec<Uuid>>,Option<Note>)) -> Self{
    let (shift_problem,problems_ids,spare_parts_ids,note) = pieaces;
    let ShiftProblem{id,machine_id,maintainer_id,shift_id,writer_id,begin_time,end_time} = shift_problem;
    MinimamlShiftProblem{ id, shift_id, writer_id, maintainer_id,
            machine_id, begin_time, end_time, problems_ids, spare_parts_ids, note }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ProblemDetail{
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
  pub problems_ids      : Vec<Uuid>,
  pub spare_parts_ids   : Option<Vec<Uuid>>,
  pub note              : Option<String>
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct ClientShiftProblem{
  pub id                : String,
  pub shift_id          : String,
  pub writer_id         : String,
  pub maintainer_id     : String,
  pub machine_id        : String,
  pub begin_time        : String,
  pub end_time          : String,
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct ClientMinimamlShiftProblem{
  pub id                : String,
  pub shift_id          : String,
  pub writer_id         : String,
  pub maintainer_id     : String,
  pub machine_id        : String,
  pub begin_time        : String,
  pub end_time          : String,
  pub problems_ids      : Vec<String>,
  pub spare_parts_ids   : Option<Vec<String>>,
  pub note              : Option<ClientNote>
}

impl ClientMinimamlShiftProblem{
  pub fn new(problem : MinimamlShiftProblem) -> Self{
    let MinimamlShiftProblem { id, shift_id, writer_id, maintainer_id, machine_id,
              begin_time, end_time, problems_ids, spare_parts_ids, note } = problem;
    let id            = id.to_string();
    let shift_id      = shift_id.to_string();
    let writer_id     = writer_id.to_string();
    let maintainer_id = maintainer_id.to_string();
    let machine_id    = machine_id.to_string();
    let begin_time    = serde_json::json!(begin_time).to_string();
    let end_time      = serde_json::json!(end_time).to_string();
    let problems_ids    : Vec<String> = problems_ids.into_iter().map(|id| id.to_string()).collect();
    let spare_parts_ids = match spare_parts_ids {
      Some(ids) => Some(ids.into_iter().map(|id| id.to_string()).collect::<Vec<String>>()),
      None      => None
    };
    let note = match note {
      Some(Note{id,content})  => Some(ClientNote{id : id.to_string(), content }),
      None                    => None
    };

    ClientMinimamlShiftProblem {
      id,
      shift_id,
      writer_id,
      maintainer_id,
      machine_id,
      begin_time,
      end_time,
      problems_ids,
      spare_parts_ids,
      note
    }
  }
  pub fn construct(pieaces : (ClientShiftProblem,Vec<String>,Option<Vec<String>>,Option<ClientNote>)) -> Self{
    let (problem,problems_ids,spare_parts_ids,note) = pieaces;
    let ClientShiftProblem{id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time} = problem;

    ClientMinimamlShiftProblem {
      id,
      shift_id,
      writer_id,
      maintainer_id,
      machine_id,
      begin_time,
      end_time,
      problems_ids,
      spare_parts_ids,
      note
    }
  }
}

impl ClientShiftProblem {
  pub fn new(sp : ShiftProblem) -> Self{
    let ShiftProblem{id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time} = sp;
    let id            = id.to_string();
    let shift_id      = shift_id.to_string();
    let writer_id     = writer_id.to_string();
    let maintainer_id = maintainer_id.to_string();
    let machine_id    = machine_id.to_string();
    let begin_time    = serde_json::json!(begin_time).to_string();
    let end_time      = serde_json::json!(end_time).to_string();
    ClientShiftProblem{id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time}
  }
}

#[derive(Serialize,Deserialize)]
pub struct WriterAndShiftIds{
  pub writer_id : Uuid,
  pub shift_id  : Uuid
}
