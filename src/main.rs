use std::net::SocketAddr;
use std::time::Duration;
use aos_dispatcher::service::nostr::model::JobAnswer;
use aos_dispatcher::ws;
use axum::{
    Router,
    routing::post,
};
use axum::error_handling::HandleErrorLayer;
use axum::handler::Handler;
use axum::http::Method;
use axum::routing::get;
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use aos_dispatcher::tee::handler::*;
use aos_dispatcher::server::server::SharedState;
use aos_dispatcher::opml::handler::*;

use tower_http::cors::{Any, CorsLayer};
use aos_dispatcher::service;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let (dispatch_task_tx, dispatch_task_rx) = mpsc::channel::<u32>(200);

    let (job_status_tx, job_status_rx) = mpsc::channel::<JobAnswer>(200);

    let config = aos_dispatcher::config::Config::new();
    let mut server = SharedState::new(config, dispatch_task_tx.clone(), job_status_tx.clone()).await;



    let nostr_sub_task = tokio::spawn(aos_dispatcher::service::nostr::subscription_service(
        server.clone(),
        job_status_rx,
        dispatch_task_tx.clone(),
    ));



    let dispatch_task = tokio::spawn(service::task::dispatch_task(
        server.clone(),
        dispatch_task_rx,
    ));

    

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/sign", post(sign))
        .route("/register_worker", post(register_worker))
        .route("/receive_heart_beat", post(receive_heart_beat))
        .route("/api/question", post(tee_question_handler))
        .route("/api/tee_callback", post(tee_callback))
        .route("/api/opml_question", post(opml_question_handler))
        .route("/api/opml_callback", post(opml_callback))
        .route("/api/list_models", post(list_models))
        .route("/admin/list_workers", post(list_workers))
        .route("/admin/list_questions", post(list_questions_handler))
        .route("/admin/list_answers", post(list_answers_handler))
        .route("/ws", get(ws::handler))
        .with_state(server)
        .layer(cors)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(600))
                .layer(TraceLayer::new_for_http())
        )
        ;

    let server_task = tokio::spawn(async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    });

    let _ = tokio::join!(
        nostr_sub_task,
        server_task,
        dispatch_task,
    );

}

