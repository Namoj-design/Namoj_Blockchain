use crate::types::{Block, Transaction, Blockchain};

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