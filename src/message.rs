use std::str::FromStr;

use crate::ws::msg::WsMethodMsg;
use alloy::{
    // primitives::keccak256,
    signers::{local::PrivateKeySigner, Signature, SignerSync},
};

pub struct MessageVerify {
    pub signer: PrivateKeySigner,
}

impl MessageVerify {
    pub fn sign_message(&self, message: &WsMethodMsg) -> anyhow::Result<WsMethodMsg> {
        let mut unsigned_message = message.clone();
        unsigned_message.address = self.signer.address().to_string();
        unsigned_message.signature = String::new();

        let msg = serde_json::to_vec(&unsigned_message)?;
        let signature = self.signer.sign_message_sync(&msg)?;

        let mut signed_message = unsigned_message;
        signed_message.signature = hex::encode(signature.as_bytes());

        Ok(signed_message)
    }

    pub fn verify_message(message: &WsMethodMsg) -> anyhow::Result<bool> {
        let sig = message.signature.as_str();
        let signature = Signature::from_str(sig)?;

        let mut origin_message = message.clone();
        origin_message.signature = String::new();

        let msg = serde_json::to_vec(&origin_message)?;
        let origin = signature.recover_address_from_msg(&msg)?;

        let address = message.address.clone();
        let addr = origin.to_string();
        let is_verify = addr.to_lowercase().eq(&address.to_lowercase());
        Ok(is_verify)
    }
}

#[cfg(test)]
mod tests {
    use alloy::signers::local::PrivateKeySigner;

    use crate::ws::msg::WsMethodMsg;

    use super::MessageVerify;

    #[test]
    fn test_verify() {
        let signer = PrivateKeySigner::from_slice(&[0x1f; 32]).expect("singer err");
        let verify = MessageVerify { signer };

        let ws_msg = WsMethodMsg {
            id: "".into(),
            method: None,
            params: None,
            result: None,
            address: "".into(),
            hash: "".into(),
            signature: "".into(),
        };

        let message = verify.sign_message(&ws_msg).expect("sign message error");

        let is_verify = MessageVerify::verify_message(&message).expect("verify message error");

        assert_eq!(is_verify, true, "");

        let mut modify_msg = message.clone();
        modify_msg.method = Some("dispatch_job".into());

        let is_verify = MessageVerify::verify_message(&modify_msg).expect("verify message error");
        assert_eq!(is_verify, false, "");
    }
}
