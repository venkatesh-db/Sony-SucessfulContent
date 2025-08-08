
use sha2::{Sha256, Digest};

fn calculate_hash(data: &str, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", data, nonce));
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn mine_block(data: &str, difficulty: usize) -> (u64, String) {
    let mut nonce = 0;
    let prefix = "0".repeat(difficulty);
    loop {
        let hash = calculate_hash(data, nonce);
        if hash.starts_with(&prefix) {
            return (nonce, hash);
        }
        nonce += 1;
    }
}

fn main() {
    let block_data = "🧱 Block: Venkatesh sends 1 BTC to Shiva";
    let difficulty = 4; // try increasing to 5 or 6 (gets slower)

    println!("Mining block with difficulty {}...", difficulty);
    let (nonce, hash) = mine_block(block_data, difficulty);
    println!("✅ Mined! Nonce: {}  Hash: {}", nonce, hash);
}
