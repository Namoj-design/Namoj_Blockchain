use crate::types::Block;

pub fn mine_block(mut block: Block, difficulty: usize) -> Block {
    let prefix = "0".repeat(difficulty);
    while !block.hash.starts_with(&prefix) {
        block.nonce += 1;
        block.hash = block.calculate_hash();
    }
    block
}