use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterProjectReq {
    pub name: String,
    pub address: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteListReq {
    pub token: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterProjectResp {
    pub code: u16,
    pub result: Value,
}
