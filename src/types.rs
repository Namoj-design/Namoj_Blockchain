use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{:?}{}",
            self.index,
            self.timestamp,
            self.prev_hash,
            self.transactions,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        hex::encode(hasher.finalize())
    }

    pub fn new(index: u64, prev_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            hash: String::new(),
            nonce: 0,
            transactions,
        };
        block.hash = block.calculate_hash();
        block
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain { chain: Vec::new(), difficulty: 2 };
        let genesis_block = Block::new(0, String::from("0"), vec![]);
        bc.chain.push(genesis_block);
        bc
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn last_hash(&self) -> String {
        self.chain.last().unwrap().hash.clone()
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        let mut block = Block::new(self.chain.len() as u64, self.last_hash(), vec![tx]);
        block.hash = block.calculate_hash();
        self.add_block(block);
    }
}