use axum::{
  extract::ws::{Message, WebSocket, WebSocketUpgrade},
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
// use axum_extra::TypedHeader;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
// use futures::{sink::SinkExt, stream::StreamExt};
use std::borrow::Cow;
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub async fn handler(
  ws: WebSocketUpgrade,
  // ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
  ws.on_upgrade(move |socket| handle_socket(socket))
}
async fn handle_socket(mut socket: WebSocket, 
  // who: SocketAddr
) {
  while let Some(msg) = socket.recv().await {
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
  }
}
