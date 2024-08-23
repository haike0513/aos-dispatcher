use axum::{
  extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
// use axum_extra::TypedHeader;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
use tokio::sync::mpsc;
// use futures::{sink::SinkExt, stream::StreamExt};
use std::{borrow::Cow, net::ToSocketAddrs, sync::Arc};
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{server::server::SharedState, service::task::DispatchTaskState};

pub async fn handler(
  ws: WebSocketUpgrade,
  State(server): State<SharedState>,
  // State(dispatch_task_state): State<DispatchTaskState>,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
  tracing::info!("{} connect", addr);

  // send client channel

  let (tx, mut rx) = mpsc::channel::<String>(20_000);
  let dispatch_tx;


  {
    let mut server = server.0.write().await;
    server.worker_channels.insert(addr.to_string(), tx);
    dispatch_tx = server.dispatch_task_tx.clone().unwrap();
  }

  
  ws.on_upgrade(move |socket| {
    handle_socket(socket, addr, rx, dispatch_tx)
  })
}
async fn handle_socket(
  mut socket: WebSocket, 
  who: SocketAddr,
  mut rx: mpsc::Receiver<String>,
  dispatch_tx:mpsc::Sender<u32>,
) {
  dispatch_tx.send(2).await.unwrap();
  tracing::info!("{} ws connect", who);
  loop {
      tokio::select!{
        // client send to dispatcher
        Some(msg) = socket.recv() => {
            let msg = if let Ok(msg) = msg {
                msg
            } else {
                // client disconnected
      
                return;
            };
      
            if socket.send(msg).await.is_err() {
                // client disconnected
                return;
            }
        },
        Some(msg) = rx.recv() => {
          tracing::debug!("send message to client");
          if socket.send(Message::Text(msg.to_string())).await.is_err() {
                // client disconnected
                return;
            }

        }

      }
  }
}
