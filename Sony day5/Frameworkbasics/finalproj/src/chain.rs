
use serde::{Serialize, Deserialize};
use sled::Db;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref BLOCKCHAIN: Mutex<Blockchain> = {
let bc = Blockchain::load().unwrap();
        Mutex::new(bc)
    };
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u32,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
struct Persist {
    chain: Vec<Block>,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    db: Db,
}

impl Blockchain {
    pub fn load() -> anyhow::Result<Self> {
        let db = sled::open("chain.db")?;
        let chain = if let Some(bytes) = db.get("chain")? {
            bincode::deserialize::<Persist>(&bytes)?.chain
        } else {
            vec![Block {
                index: 0,
                data: "Genesis".into(),
                prev_hash: String::new(),
                hash: "0".into(),
            }]
        };
        Ok(Blockchain { chain, db })
    }

    pub fn add_block(&mut self, data: String) {
        let prev = self.chain.last().unwrap();
        let index = prev.index + 1;
        let prev_hash = prev.hash.clone();
        let hash = blake3::hash(format!("{}{}{}", index, data, prev_hash).as_bytes()).to_hex().to_string();
     //   let hash = format!("{:x}", blake3::hash(format!("{}{}{}", index, &data, &prev_hash))) ;
        let b = Block { index, data, prev_hash, hash };
        self.chain.push(b);
        let persist = Persist { chain: self.chain.clone() };
        self.db.insert("chain", bincode::serialize(&persist).unwrap()).unwrap();
    }

    pub fn replace(&mut self, other: Vec<Block>) {
        if other.len() > self.chain.len() {
            self.chain = other;
            let persist = Persist { chain: self.chain.clone() };
            self.db.insert("chain", bincode::serialize(&persist).unwrap()).unwrap();
        }
    }
}
