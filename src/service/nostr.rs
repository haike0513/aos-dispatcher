const MNEMONIC_PHRASE: &str = "equal dragon fabric refuse stable cherry smoke allow alley easy never medal attend together lumber movie what sad siege weather matrix buffalo state shoot";
const DEFAULT_RELAY: &str = "ws://localhost:7010";

use nostr::nips::nip06::FromMnemonic;
use nostr::nips::nip19::ToBech32;
use nostr::{Keys, Result};

use nostr_sdk::{Client, Event, EventBuilder, Filter, Kind, Metadata, RelayPoolNotification, SingleLetterTag, Tag, TagKind, Url};
use tokio::sync::mpsc;
use tracing::instrument::WithSubscriber;

use crate::opml::model::{create_opml_question, OpmlRequest};
use crate::server::server::SharedState;
use crate::tee::model::{create_question, OperatorReq};
pub mod util;
pub async fn subscription_service(
  server: SharedState,
  mut job_status_rx: mpsc::Receiver<u32>,
){
  let keys = Keys::from_mnemonic(MNEMONIC_PHRASE, None).unwrap();
  let bech32_address = keys.public_key().to_bech32().unwrap();

  let client = Client::new(&keys);
  // let client = Client::default();
  client.add_relay(DEFAULT_RELAY).await.unwrap();
  client.connect().await;
  tracing::info!("connect relay {:#?} with {:#?}", DEFAULT_RELAY, bech32_address);
  let metadata = Metadata::new()
  .name("aos-dispatcher")
  .display_name("Aos Dispatcher")
  .website(Url::parse("https://github.com/hetu-project/aos-dispatcher").unwrap());

  let submit_client = client.clone();
  let job_status_submit = tokio::spawn(async move {

    while let Some(job_status) = job_status_rx.recv().await {
      tracing::info!("job status {:#?}", job_status);
      let tags: Vec<Tag> = vec![];
      let event = EventBuilder::text_note("content", tags);
      submit_client.send_event_builder(event).await.unwrap();
    }
  });

  let subscription = Filter::new()
  // .pubkey(keys.public_key())
  .kinds([Kind::JobRequest(5050)])
  // .kind(Kind::Custom(5050))
  // .limit(10)
  ;

  client.subscribe(vec![subscription], None).await.unwrap();
  tracing::info!("Subscription ID: [auto-closing] start");

  let sub = client.handle_notifications(|notification|async{
    tracing::debug!("job notification {:#?}", notification);
    if let RelayPoolNotification::Event{
      event, ..
    }  = notification {
      // tracing::info!("job notification {:#?}", event);
      if event.kind() == Kind::JobRequest(5050) {

        tracing::info!("receive task {:#?}", event);
        tracing::info!("receive task {:#?}", event.id());
        // let uuid = uuid::Uuid::new_v4();
        let request_id =  event.id().to_string();
        // let mut e_model = None;
        // let mut e_prompt = None;

        let aos_task = util::AosTask::parse_event(&event).unwrap();
        tracing::debug!("dispatch task start {:#?}", request_id);

        {

          let mut server = server.0.write().await;
          let mut conn = server.pg.get().expect("Failed to get a connection from pool");
          let message = aos_task.prompt.unwrap_or_default();
          let message_id = event.id().to_string();
          let conversation_id = event.id().to_string();
          let model = aos_task.model.unwrap_or_default();
          let callback_url = event.id().to_string();

          // TODO: dispatch task to worker by websocket
          // dispatch_task_rx.send(2).await.unwrap();

          
          // start dispatch tee task
          tracing::debug!("dispatch tee task {:#?}", request_id);
          let q = create_question(
            &mut conn, 
            request_id.clone(),
            message.clone(),
            message_id,
            conversation_id,
            model.clone(), 
            callback_url.clone(),
          );

          let op_req = OperatorReq {
            request_id: q.request_id.clone(),
            node_id: "".to_string(),
            model: model.clone(),
            prompt: message.clone(),
            prompt_hash: "".to_string(),
            signature: "".to_string(),
            params: aos_task.params.clone(),
        };
          let work_name = server.tee_operator_collections.keys().next().unwrap().clone();
          server.send_tee_inductive_task(work_name, op_req).await;


          tracing::debug!("dispatch opml task {:#?}", request_id);
          let opml_request = OpmlRequest {
            model: model.clone(),
            prompt: message.clone(),
            req_id: request_id.clone(),
            callback: callback_url.clone(),
        };

        if let Err(e) = create_opml_question(&mut conn, request_id.clone(), &opml_request) {
          tracing::error!("Failed to store OPML question: {:?}", e);
        }


        // Send the request to the OPML server
        if let Err(e) = server.send_opml_request(opml_request).await {
          tracing::error!("Failed to send OPML request: {:?}", e);
        }
          
        }
        tracing::debug!("dispatch task end {:#?}", request_id);
      } else {
        tracing::info!("JobRequest other {:#?}", event.kind());
      }

    }
    Ok(false)
  }).await.unwrap();

  tracing::info!("Subscription ID: [auto-closing] end {:#?}", sub);
  job_status_submit.await.unwrap();
}