use axum::{BoxError, debug_handler, extract, Json};
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::db::pg::model::Operator;
use crate::server::server::SharedState;
use serde_json::json;
use crate::db::pg;

use crate::operator::model::{OperatorRegisterReq, OperatorRegisterResp};

#[debug_handler]
pub async fn register(State(server): State<SharedState>, Json(req): Json<OperatorRegisterReq>) -> Json<serde_json::Value> {
  tracing::debug!("register operator");
  let id: String = req.address.to_string();
  let operator = Operator { 
    id: id.clone(),
    name: "".into(), 
    address: "".into(),
    start: "".into(),
    end: "".into(), 
    operator_type: "".into(),
    status: "".into(), 
    created_at: chrono::Local::now().naive_local(), 
  };
  let list = vec![operator];
  let mut server = server.0.write().await;
  let keys = &server.nostr_keys;
  let mut conn = server.pg.get().expect("Failed to get a connection from pool");
  let insert_operators = pg::util::sync_operators_info(&mut conn, &list).expect("Error saving new question");
  Json(json!({
    "code": 200,
    "result": insert_operators,
}))
}