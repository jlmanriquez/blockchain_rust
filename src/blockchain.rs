use crate::block::Block;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    difficulty: u32,
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        Blockchain {
            difficulty,
            chain: vec![Block::new("Genesis Block")],
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let mut new_block = Block::new_from_prev(self.chain.last().unwrap(), data);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn get_block(&self) -> &[Block] {
        self.chain.as_slice()
    }
}