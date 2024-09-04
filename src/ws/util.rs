use super::msg::WsMethodMsg;

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
