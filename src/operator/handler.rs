use axum::extract::State;
use axum::{debug_handler, Json};
use rand::Rng;

use crate::db::pg::model::Operator;
use crate::db::pg::util::{create_operator, get_operator_by_id};
use crate::operator::util::register_operator;
use crate::server::server::SharedState;
use serde_json::json;

use crate::operator::model::{OperatorInfoReq, OperatorRegisterReq};

pub fn sample_range_of_operators(ops: &mut Vec<Operator>) {
    let count = ops.len();
    let range_max = 0xffffff;
    let range_min = 0x000000;
    let diff = range_max - range_min;
    // let range_per = (range_max - range_min) / count;
    for (pos, op) in ops.iter_mut().enumerate() {
        let start = range_min + ((pos * diff) / count);
        let end = range_min + (((pos + 1) * diff) / count);
        op.start = format!("{:06x}", start).into();
        op.end = format!("{:06x}", end).into();
    }
}

pub fn sample_range_of_operator(_ops: &Operator, min: u32, max: u32, count: u32) -> (u32, u32) {
    let range_max = max;
    let range_min = min;
    let diff = range_max - range_min;
    let mut rng = rand::thread_rng();
    let pos: u32 = rng.gen_range(0..count);
    let start = range_min + ((pos * diff) / count);
    let end = range_min + (((pos + 1) * diff) / count);
    (start, end)
}

#[debug_handler]
pub async fn register(
    State(server): State<SharedState>,
    Json(req): Json<OperatorRegisterReq>,
) -> Json<serde_json::Value> {
    tracing::debug!("register operator");
    let id: String = req.address.to_string();
    let mut operator = Operator {
        id: req.params.operator.clone(),
        name: format!("Operator {}", &id).into(),
        address: req.params.operator.clone(),
        start: "".into(),
        end: "".into(),
        operator_type: "".into(),
        status: "".into(),
        created_at: chrono::Local::now().naive_local(),
    };
    let server = server.0.write().await;
    let _keys = &server.nostr_keys;
    let mut conn = match server.pg.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get a database connection: {:?}", e);
            return Json(json!({
                "code": 500,
                "message": "",
            }));
        }
    };
    let sample_range = sample_range_of_operator(&operator, 0, 6000, 10);
    operator.start = sample_range.0.to_string();
    operator.end = sample_range.1.to_string();
    tracing::debug!("operator {:#?} {:#?}", &operator.start, &operator.end);
    register_operator(
        &operator,
        sample_range.0,
        sample_range.1,
        &server.config.custom_config,
    )
    .await
    .unwrap();
    let r = create_operator(&mut conn, &operator);

    if let Err(e) = r {
        return Json(json!({
          "code": 500,
          "message": e.to_string(),
          "result": null
        }));
    }

    // let list = vec![operator];
    // let mut list = query_operators(&mut conn).unwrap();
    // sample_range_of_operators(&mut list);
    // let insert_operators = pg::util::sync_operators_info(&mut conn, &list).expect("Error saving new question");
    Json(json!({
        "code": 200,
        "result": operator,
    }))
}

#[debug_handler]
pub async fn operator_info(
    State(server): State<SharedState>,
    Json(req): Json<OperatorInfoReq>,
) -> Json<serde_json::Value> {
    tracing::debug!("operator info");
    let operator_id = req.operator;
    let server = server.0.write().await;
    // let keys = &server.nostr_keys;
    let mut conn = match server.pg.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get a database connection: {:?}", e);
            return Json(json!({
                "code": 500,
                "message": "",
            }));
        }
    };
    let operator = get_operator_by_id(&mut conn, &operator_id).ok();
    Json(json!({
        "code": 200,
        "result": operator,
    }))
}
