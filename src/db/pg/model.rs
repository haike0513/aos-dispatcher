use std::collections::HashMap;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use reqwest::{Client, Url};
use std::time::Duration;
use crate::schema::answers::dsl::*;
use crate::schema::questions;
use crate::schema::answers;
use crate::schema::questions::dsl::{request_id, questions as questions_table};
use crate::schema::answers::dsl::{request_id as answer_request_id};

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



#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Question {
    pub request_id: String,
    pub message: String,
    pub message_id: String,
    pub conversation_id: String,
    pub model: String,
    pub callback_url: String,
    pub job_type: String,
    pub status: String,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}
