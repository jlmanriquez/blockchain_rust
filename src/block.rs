use std::time::{SystemTime};
use crypto_hash::{Algorithm, hex_digest};

#[derive(Debug)]
pub struct Block {
    prev_hash: String,
    index: u32,
    nonce: i64,
    data: String,
    hash: String,
    timestamp: u128,
}

impl Block {
    pub fn new(index: u32, data: &str) -> Self {
        let mut b = Block {
            prev_hash: String::from(""),
            index,
            nonce: -1,
            data: String::from(data),
            hash: String::from(""),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        };

        b.calculate_hash();
        b
    }

    pub fn get_hash(&self) -> &str {
       &self.hash[..]
    }

    pub fn set_prev_hash(&mut self, h: &str) {
        self.prev_hash = String::from(h);
    }

    pub fn mine_block(&mut self, difficulty: u32) {
        let mut chars = Vec::new();

        for _ in 0..(difficulty) {
            chars.push(b'0');
        }

        let str = String::from_utf8(chars).unwrap();

        loop {
            self.nonce += 1;
            self.calculate_hash();

            if self.hash[0..difficulty as usize] == str {
                break;
            }
        }
    }

    fn calculate_hash(&mut self) {
        let buf = format!(
            "{}{:?}{}{}{}", self.index, self.prev_hash, self.timestamp, self.data, self.nonce);

        self.hash = hex_digest(Algorithm::SHA256, buf.as_bytes());
    }
}
