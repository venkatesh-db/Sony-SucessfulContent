
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: String,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let hash = Block::calculate_hash(index, &timestamp, &data, &prev_hash);

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: &str, data: &str, prev_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, prev_hash));
        format!("{:x}", hasher.finalize())
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block::new(0, "Genesis Block".into(), String::new());
        Blockchain {
            chain: vec![genesis],
        }
    }

    fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}

fn main() {
    let mut my_chain = Blockchain::new();
    my_chain.add_block("Venkatesh sent 10 coins to Ravi".into());
    my_chain.add_block("Ravi sent 5 coins to Sneha".into());

    println!("🧱 Blockchain Structure:");
    my_chain.print_chain();
}
