use crate::block::Block;

pub struct Blockchain {
    difficulty: u32,
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        Blockchain {
            difficulty,
            chain: vec![Block::new(0, "Initial Block")]
        }
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.set_prev_hash(self.get_last_block().unwrap().get_hash());
        block.mine_block(self.difficulty);

        self.chain.push(block);
    }

    fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn get_size(&self) -> usize { self.chain.len() }
}