use crate::types::Block;
use crate::chain::Blockchain;

pub fn mine_block(bc: &mut Blockchain, miner_addr: String) -> Option<Block> {
    if bc.mempool.is_empty() { return None; }
    let mut block = Block {
        index: bc.chain.len() as u64,
        prev_hash: bc.last_block().hash(),
        timestamp: chrono::Utc::now().timestamp_millis() as u128,
        transactions: bc.mempool.clone(),
        nonce: 0,
    };
    // add reward tx
    block.transactions.push(crate::types::Transaction { from: "0".to_string(), to: miner_addr, amount: 50, signature: "".to_string() });

    let target = "0".repeat(bc.difficulty);
    loop {
        let h = block.hash();
        if h.starts_with(&target) {
            bc.mempool.clear();
            bc.chain.push(block.clone());
            return Some(block);
        }
        block.nonce += 1;
    }
}
