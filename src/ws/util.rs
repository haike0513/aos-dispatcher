use axum::extract::ws::Message;
use tokio::sync::mpsc;

use crate::server::server::SharedState;

use super::msg::{ConnectParams, WsMethodMsg};

pub fn convert_to_msg(msg: &str) -> Result<WsMethodMsg, ()> {
  let method_msg =
      serde_json::from_str::<WsMethodMsg>(msg);
  match method_msg {
      Ok(m) => {
          Ok(m)
      }
      Err(e) => {
          Err(())
      }
  }
}

pub async fn connect_to_dispatcher(
    msg: &WsMethodMsg,
    mut tx: mpsc::Sender<Message>,
    server: SharedState,
) -> Result<(), ()>{
    let operator = msg.params.as_array().and_then(|p| {
        let a = p.get(0);
        if let Some(s) = a {
            let p = serde_json::from_value::<ConnectParams>(s.clone()).ok();
            return  p
        }
        None
    });
    if let Some(p) = operator {
        tracing::debug!("operator id {} connect", p.operator);
        let mut server = server.0.write().await;
        server.operator_channels.insert(p.operator, tx);
    }
    Ok(())
}
