use crate::server::server::SharedState;
use crate::service::nostr::model::JobAnswer;
use crate::tee::model::list_questions;
use crate::tee::model::*;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, BoxError, Json};
use nostr_sdk::EventId;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

#[debug_handler]
pub async fn sign(
    State(server): State<SharedState>,
    Json(req): Json<HashRequest>,
) -> Json<HashResponse> {
    let message: &[u8] = req.hash.as_bytes();
    let server = server.0.read().await;
    let signature = server.sign(message);
    let response = HashResponse {
        sig: signature.to_string(),
    };
    Json(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse {
    code: u16,
    result: String,
}
#[debug_handler]
pub async fn register_worker(
    State(server): State<SharedState>,
    Json(req): Json<Operator>,
) -> Json<RegisterResp> {
    tracing::info!("Registering worker {:?}", req);
    let mut server = server.0.write().await;
    server.add_worker(
        req.worker_name.clone(),
        req.check_heart_beat,
        req.worker_status.clone(),
        req.multimodal,
    );

    let response = RegisterResp {
        code: 200,
        result: "ok".to_string(),
    };
    Json(response)
}

#[debug_handler]
pub async fn receive_heart_beat(
    State(server): State<SharedState>,
    Json(req): Json<HeartBeatReq>,
) -> Json<HeartBeatResp> {
    tracing::info!("Receiving heart beat {:?}", req);
    let server = server.0.write().await;
    let exist = server
        .tee_operator_collections
        .contains_key(&req.worker_name);
    let response = HeartBeatResp { exist };
    Json(response)
}

#[debug_handler]
pub async fn tee_callback(
    State(server): State<SharedState>,
    Json(req): Json<AnswerReq>,
) -> Json<AnswerResp> {
    tracing::info!("tee_callback function triggered: {:?}", req);

    let server = server.0.read().await;
    let mut conn = match server
        .pg
        .get() {
            Ok(c) => {
                c
            },
            Err(_) => {
                let response = AnswerResp {
                    code: 500,
                    result: "".to_string(),
                };
                return Json(response);
            },
        };

    let event_id = match EventId::from_str(&req.request_id) {
        Ok(id) => {
            id
        },
        Err(_) => {
            EventId::all_zeros()
        },
    };

    if let Some(job_status_tx) = server.job_status_tx.clone() {
        if let Err(err) = job_status_tx
            .send(JobAnswer {
                event_id: event_id,
                answer: req.answer.clone(),
            })
            .await {
                tracing::error!("send job  answer error err {}", err);
            }
    }

    match create_tee_answer(&mut conn, &req) {
        Ok(_) => {
            // Forward the answer to the callback URL
            if let Some(tx) = server.tee_channels.get(&req.request_id) {
                tracing::info!(
                    "Sending answer through channel, request_id: {}",
                    req.request_id
                );
                if let Err(e) = tx.send(req.clone()).await {
                    tracing::error!("Failed to send OPML answer through channel: {:?}", e);
                }
            }

            let response = AnswerResp {
                code: 200,
                result: "Callback stored successfully".to_string(),
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Failed to store callback: {:?}", e);
            let response = AnswerResp {
                code: 500,
                result: "Failed to store callback".to_string(),
            };
            Json(response)
        }
    }
}

pub async fn list_models(State(state): State<SharedState>) -> axum::Json<Vec<String>> {
    let server = state.0.read().await;
    let unique_models: Vec<String> = server
        .tee_operator_collections
        .values()
        .flat_map(|operator| operator.worker_status.model_names.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    Json(unique_models)
}

pub async fn list_workers(State(state): State<SharedState>) -> axum::Json<Vec<String>> {
    let server = state.0.read().await;
    let workers: Vec<String> = server.tee_operator_collections.keys().cloned().collect();
    Json(workers)
}

pub async fn list_questions_handler(State(server): State<SharedState>) -> Json<ListQuestionsResp> {
    let server = server.0.read().await;
    let mut conn = match server.pg.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get a database connection: {:?}", e);
            return Json(ListQuestionsResp {
                code: 500,
                result: vec![],
            });
        }
    };

    match list_questions(&mut conn) {
        Ok(questions) => {
            let response = ListQuestionsResp {
                code: 200,
                result: questions,
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Failed to list questions: {:?}", e);
            let response = ListQuestionsResp {
                code: 500,
                result: vec![],
            };
            Json(response)
        }
    }
}

pub async fn list_answers_handler(State(server): State<SharedState>) -> Json<ListAnswersResp> {
    let server = server.0.read().await;
    let mut conn = match server.pg.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get a database connection: {:?}", e);
            return Json(ListAnswersResp {
                code: 500,
                result: vec![],
            });
        }
    };

    match list_answers(&mut conn) {
        Ok(answers) => {
            let response = ListAnswersResp {
                code: 200,
                result: answers,
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Failed to list answers: {:?}", e);
            let response = ListAnswersResp {
                code: 500,
                result: vec![],
            };
            Json(response)
        }
    }
}
pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}
