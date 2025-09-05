use crate::types::{Block, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain { chain: vec![], mempool: vec![], difficulty: 4 };
        bc.create_genesis();
        bc
    }

    fn create_genesis(&mut self) {
        let genesis = Block { index: 0, prev_hash: "0".to_string(), timestamp: chrono::Utc::now().timestamp_millis() as u128, transactions: vec![], nonce: 0 };
        self.chain.push(genesis);
    }

    pub fn last_block(&self) -> &Block { &self.chain[self.chain.len() - 1] }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.mempool.push(tx);
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        // basic checks
        if block.prev_hash != self.last_block().hash() { return false; }
        if !block.hash().starts_with(&"0".repeat(self.difficulty)) { return false; }
        self.chain.push(block);
        true
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let cur = &self.chain[i];
            let prev = &self.chain[i-1];
            if cur.prev_hash != prev.hash() { return false; }
            if !cur.hash().starts_with(&"0".repeat(self.difficulty)) { return false; }
        }
        true
    }
}