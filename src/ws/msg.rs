use axum::extract::ws::Message;

 pub enum WsSendMsg {
  Ping
 }


 impl  Into<Message> for  WsSendMsg {
    fn into(self) -> Message {
      Message::Text("()".into())
    }
 }