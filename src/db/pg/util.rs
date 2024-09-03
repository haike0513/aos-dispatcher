use chrono::NaiveDateTime;
use diesel::upsert::excluded;
use serde::Deserialize;
use diesel::prelude::*;

use crate::schema::answers;
use crate::db::pg::model::{Question, Answer};
use crate::schema::answers::dsl::{request_id as answer_request_id};

use super::model::Operator;


pub fn serialize_naive_datetime<S>(
  date: &NaiveDateTime,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  let s = date.format("%Y-%m-%d %H:%M:%S").to_string();
  serializer.serialize_str(&s)
}

pub fn deserialize_naive_datetime<'de, D>(
  deserializer: D,
) -> Result<NaiveDateTime, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}


pub fn create_question(conn: &mut PgConnection, q: &Question) -> Result<Question, diesel::result::Error> {


  diesel::insert_into(crate::schema::questions::table)
      .values(q)
      .returning(Question::as_returning())
      .get_result(conn)
      // .expect("Error saving new question")
}

pub fn get_answer_by_id(conn: &mut PgConnection, q_id: &str) -> Result<Option<Answer>, diesel::result::Error> {
  answers::table
      .filter(answer_request_id.eq(q_id))
      .first::<Answer>(conn)
      .optional()
}

pub fn sync_operators_info(conn: &mut PgConnection, operators: &Vec<Operator>) -> Result<Vec<Operator>, diesel::result::Error> {


  diesel::insert_into(crate::schema::operator::table)
      .values(operators)
      // .on_conflict(target)
      .on_conflict(crate::schema::operator::id)
      .do_update()
      .set(
        (
          crate::schema::operator::start.eq(excluded(crate::schema::operator::start)),
          crate::schema::operator::end.eq(excluded(crate::schema::operator::end)),
        )
      )
      .returning(Operator::as_returning())
      .get_results(conn)
      // .expect("Error saving new question")
}