use crate::block::Block;
use std::sync::{Mutex};

pub struct Blockchain {
    difficulty: u32,
    chain: Mutex<Vec<Block>>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        Blockchain {
            difficulty,
            chain: Mutex::new(vec![Block::new("Genesis Block")]),
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let mut chain = self.chain.lock().unwrap();

        let mut new_block = Block::new_from_prev(chain.last().unwrap(), data);
        new_block.mine_block(self.difficulty);

        chain.push(new_block);
    }

    pub fn iter<F>(&self, action: F) where F: Fn(&Block) {
        let data = self.chain.lock().unwrap();
        for b in data.iter() {
            action(b);
        }
    }
}