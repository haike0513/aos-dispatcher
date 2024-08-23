use std::sync::Arc;

use axum::extract::FromRef;
use tokio::sync::{mpsc, RwLock};

use crate::server::server::SharedState;

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

pub async fn dispatch_task(
  server: SharedState,
  mut rx: mpsc::Receiver<u32>,
) {
  while let Some(i) = rx.recv().await {
    tracing::info!("start dispatch task {}", i);
    let server = server.0.write().await;
    let keys = server.worker_channels.keys();
    if let Some(k) = keys.into_iter().next() {
      let worker_tx = server.worker_channels.get(k);
      if let Some(tx) =  worker_tx {
          tx.send("value".into()).await.unwrap();
          tracing::debug!("start dispatch task to {}", k); 

          // TODO: Mock
          tracing::debug!("mock receive job status"); 
          let job_status_tx = server.job_status_tx.clone().unwrap();
          job_status_tx.send(1).await.unwrap();
          tracing::debug!("mock send job status"); 

      }else {
        tracing::info!("start dispatch task error {} for no worker connect", i);  
      }
    } else {
      tracing::info!("start dispatch task error {} for no worker connect", i);  
    }
  }

}