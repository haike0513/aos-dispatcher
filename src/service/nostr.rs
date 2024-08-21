const MNEMONIC_PHRASE: &str = "equal dragon fabric refuse stable cherry smoke allow alley easy never medal attend together lumber movie what sad siege weather matrix buffalo state shoot";
const DEFAULT_RELAY: &str = "ws://localhost:7010";

use nostr::nips::nip06::FromMnemonic;
use nostr::nips::nip19::ToBech32;
use nostr::{Keys, Result};

use nostr_sdk::{Client, Filter, Kind, Metadata, RelayPoolNotification, SingleLetterTag, TagKind, Url};

use crate::server::server::SharedState;
use crate::tee::model::create_question;

pub async fn subscription_service(
  server: SharedState,
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
        let mut e_model = None;
        let mut e_prompt = None;

        {
          let model_tag = event.tags.iter().find(|t| { 
            if t.kind() != TagKind::Custom("param".into()) {
              return  false;
            }
            let content = t.as_vec();
            if let Some(p) = content.get(1) {
              if p.eq(&String::from("model")) {
                e_model = content.get(2).map(|m| {
                  m.clone()
                })
              }
              ;
            }
            return  false;
          });

          let _ = event.tags.iter().find(|t| { 
            if t.kind() != TagKind::SingleLetter(SingleLetterTag::lowercase(nostr_sdk::Alphabet::I)) {
              return  false;
            }
            let content = t.as_vec();
            if let Some(p) = content.get(2) {
              if p.eq(&String::from("prompt")) {
                e_prompt = content.get(1).map(|m| {
                  m.clone()
                })
              }
              ;
            }
            return  false;
          });
          
          // tracing::debug!("receive task model {:#?}", e_model);
          // tracing::debug!("receive task e_prompt {:#?}", e_prompt);


          let mut server = server.0.write().await;
          let mut conn = server.pg.get().expect("Failed to get a connection from pool");
          let message = e_prompt.unwrap_or_default();
          let message_id = event.id().to_string();
          let conversation_id = event.id().to_string();
          let model = e_model.unwrap_or_default();
          let callback_url = event.id().to_string();

          let q = create_question(
            &mut conn, 
            request_id.clone(),
            message,
            message_id,
            conversation_id,
            model, 
            callback_url
          );
        }

        tracing::debug!("JobRequest 5050 {:#?}", event);

        tracing::info!("JobRequest 5050 {:#?}", event.kind());
      } else {
        tracing::info!("JobRequest other {:#?}", event.kind());
      }

    }
    Ok(false)
  }).await.unwrap();

  tracing::info!("Subscription ID: [auto-closing] end {:#?}", sub);
}