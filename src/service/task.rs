use std::{str::FromStr, sync::Arc};

use axum::extract::FromRef;
use nostr_sdk::EventId;
use tokio::sync::{mpsc, RwLock};

use crate::{
    opml::model::OpmlRequest,
    server::server::SharedState,
    service::nostr::{model::JobAnswer, util::query_question},
    tee::model::{OperatorReq, Params},
};

#[derive(Debug, Clone)]
pub struct DispatchTaskState(pub(crate) Arc<RwLock<DispatchTask>>);

impl DispatchTaskState {
    pub fn new(tx: mpsc::Sender<u32>) -> Self {
        Self(Arc::new(RwLock::new(DispatchTask {
            dispatch_task_tx: tx,
        })))
    }
}

#[derive(Debug, Clone, FromRef)]
pub struct DispatchTask {
    pub dispatch_task_tx: mpsc::Sender<u32>,
}

pub async fn dispatch_task(server: SharedState, mut rx: mpsc::Receiver<u32>) {
    while let Some(i) = rx.recv().await {
        tracing::info!("start dispatch task {}", i);
        let server = server.0.write().await;
        let mut conn = server
            .pg
            .get()
            .expect("Failed to get a connection from pool");
        let questions = query_question(&mut conn).unwrap_or_default();
        let dispatch_question = questions.iter().next();

        // dispatch the question by call operator api

        if let Some(q) = dispatch_question {
            let op_req = OperatorReq {
                request_id: q.request_id.clone(),
                node_id: "".to_string(),
                model: q.model.clone(),
                prompt: q.message.clone(),
                prompt_hash: "".to_string(),
                signature: "".to_string(),
                params: Params {
                    temperature: 0.0,
                    top_p: 0.1,
                    max_tokens: 1024,
                },
            };

            let work_name = server
                .tee_operator_collections
                .keys()
                .next()
                .unwrap()
                .clone();
            server.send_tee_inductive_task(work_name, op_req).await;

            // Send the request to the OPML server

            let opml_request = OpmlRequest {
                model: q.model.clone(),
                prompt: q.message.clone(),
                req_id: q.request_id.clone(),
                callback: "".into(),
            };
            if let Err(e) = server.send_opml_request(opml_request).await {
                tracing::error!("Failed to send OPML request: {:?}", e);
            }

            // dispatch the question by websocket
            let keys = server.worker_channels.keys();
            if let Some(k) = keys.into_iter().next() {
                let worker_tx = server.worker_channels.get(k);
                if let Some(tx) = worker_tx {
                    if let Some(q) = dispatch_question {
                        tx.send(q.message.clone()).await.unwrap();
                        tracing::debug!("start dispatch task to {}", k);

                        // TODO: Mock
                        tracing::debug!("mock receive job status");
                        let job_status_tx = server.job_status_tx.clone().unwrap();
                        let job_answer = JobAnswer {
                            event_id: EventId::from_str(&q.message_id).unwrap(),
                        };
                        tracing::debug!("start send job status {:#?}", job_answer);
                        job_status_tx.send(job_answer).await.unwrap();
                    } else {
                        tracing::debug!("There is no question wait to dispatch");
                    }
                } else {
                    tracing::info!("start dispatch task error {} for no worker connect", i);
                }
            } else {
                tracing::info!("start dispatch task error {} for no worker connect", i);
            }
        }
    }
}
