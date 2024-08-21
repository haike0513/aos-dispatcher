const MNEMONIC_PHRASE: &str = "equal dragon fabric refuse stable cherry smoke allow alley easy never medal attend together lumber movie what sad siege weather matrix buffalo state shoot";
const DEFAULT_RELAY: &str = "ws://localhost:7010";

use nostr::nips::nip06::FromMnemonic;
use nostr::nips::nip19::ToBech32;
use nostr::{Keys, Result};

use nostr_sdk::{Client, Filter, Kind, Metadata, Url, RelayPoolNotification};

pub async fn subscription_service(){
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
        tracing::info!("JobRequest 5050 {:#?}", event.kind());
      } else {
        tracing::info!("JobRequest other {:#?}", event.kind());
      }

    }
    Ok(false)
  }).await.unwrap();

  tracing::info!("Subscription ID: [auto-closing] end {:#?}", sub);
}