use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use hex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String, // hex pubkey
    pub to: String,   // hex pubkey
    pub amount: u64,
    pub signature: String, // hex signature (ed25519)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub prev_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn hash(&self) -> String {
        let json = serde_json::to_string(self).expect("serialize block");
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        hex::encode(hasher.finalize())
    }
}
