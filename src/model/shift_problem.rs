use chrono::NaiveTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::note::Note;

#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct MinimamlShiftProblem{
  pub id                : Option<Uuid>,
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

impl MinimamlShiftProblem {
  pub fn new(problem : ProblemDetail) -> Self{
    let ProblemDetail{
      id,
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

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ProblemDetail{
  pub id                : Option<Uuid>,
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
pub struct DbShiftProblem{
  pub id                : Option<Uuid>,
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
}

#[derive(Serialize,Deserialize)]
pub struct WriterAndShiftIds{
  pub writer_id : Uuid,
  pub shift_id  : Uuid
}
