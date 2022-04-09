use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::blockchain::*;

#[derive(Debug)]
pub struct Master {
    pub master_blocks: Vec<Master_Block>,
}

#[derive(Debug, Clone)]
pub struct Master_Block {
    pub id: i64,
    pub previous_hash: String,
    pub block_hash: String,
    pub block_data: Vec<Block>,
}

impl Master {
    pub fn new() -> Self {
        Self {
            master_blocks: vec![Master_Block::new(
                0,
                "genesus".to_string(),
                [Block {
                    id: -1,
                    previous_hash: "abcd".to_string(),
                    block_hash: "efgh".to_string(),
                    transaction: Transaction {
                        sender: "test".to_string(),
                        reciever: "net".to_string(),
                        amount: 10.0,
                    },
                }]
                .to_vec(),
            )],
        }
    }

    pub fn add_master_block(&mut self, block_data: Vec<Block>) {
        let prev_block = &self.master_blocks[&self.master_blocks.len() - 1];
        let new_block =
            Master_Block::new(prev_block.id + 1, prev_block.clone().block_hash, block_data);
        self.master_blocks.push(new_block);
    }

    pub fn validate_master_block(
        &self,
        block: &Master_Block,
        previous_block: &Master_Block,
    ) -> bool {
        if previous_block.block_hash.trim() == block.previous_hash.trim() {
            true
        } else {
            false
        }
    }

    pub fn validate_chain(&self) {
        for i in 1..self.master_blocks.len() {
            let block = &self.master_blocks[i];
            let prev_block = &self.master_blocks[i - 1].clone();
            if self.validate_master_block(block, prev_block) == false {
                println!("block bad");
                return;
            }
        }
        println!("Validated All Blocks");
    }

    pub fn find_master_block_by_hash(&self, hash: String) -> Master_Block {
        for block in self.master_blocks.clone() {
            if block.block_hash.trim() == hash.trim() {
                return block;
            }
        }

        Master_Block {
            id: 0,
            block_hash: "null".to_string(),
            previous_hash: "null".to_string(),
            block_data: [Block {
                id: -1,
                previous_hash: "abcd".to_string(),
                block_hash: "efgh".to_string(),
                transaction: Transaction {
                    sender: "test".to_string(),
                    reciever: "net".to_string(),
                    amount: 10.0,
                },
            }]
            .to_vec(),
        }
    }
}

impl Master_Block {
    pub fn new(id: i64, previous_hash: String, block_data: Vec<Block>) -> Master_Block {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}-{:?}", id, previous_hash, block_data));
        let block_hash_str: String = format!("{:x}", hasher.finalize());
        Master_Block {
            id: id,
            previous_hash: previous_hash,
            block_hash: block_hash_str,
            block_data: block_data,
        }
    }
}
