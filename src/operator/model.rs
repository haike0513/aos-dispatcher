use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegisterReq {
    pub address: Value,
    pub signature: Value,
    pub params: Value,
}

#[derive(Serialize, Deserialize)]
pub struct OperatorRegisterResp {
    pub code: u16,
    pub message: Value,
    pub result: Value,
}