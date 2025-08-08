
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
struct TxOutput {
    to: String,
    amount: u64,
    id: String, // UTXO id
}

#[derive(Debug)]
struct TxInput {
    utxo_id: String,
}

#[derive(Debug)]
struct Transaction {
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}

struct Blockchain {
    utxos: HashMap<String, TxOutput>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            utxos: HashMap::new(),
        }
    }

    fn create_genesis(&mut self, to: &str, amount: u64) {
        let output = TxOutput {
            to: to.to_string(),
            amount,
            id: Uuid::new_v4().to_string(),
        };
        self.utxos.insert(output.id.clone(), output);
    }

    fn create_transaction(&mut self, from: &str, to: &str, amount: u64) -> Option<Transaction> {
        // Step 1: Gather UTXOs
        let mut total = 0;
        let mut inputs = vec![];
        let mut used_utxos = HashSet::new();

        for (id, utxo) in &self.utxos {
            if utxo.to == from && !used_utxos.contains(id) {
                total += utxo.amount;
                inputs.push(TxInput {
                    utxo_id: id.clone(),
                });
                used_utxos.insert(id.clone());
                if total >= amount {
                    break;
                }
            }
        }

        if total < amount {
            println!("Not enough funds for {}", from);
            return None;
        }

        // Step 2: Remove spent UTXOs
        for input in &inputs {
            self.utxos.remove(&input.utxo_id);
        }

        // Step 3: Create outputs
        let mut outputs = vec![TxOutput {
            to: to.to_string(),
            amount,
            id: Uuid::new_v4().to_string(),
        }];

        // Change
        if total > amount {
            outputs.push(TxOutput {
                to: from.to_string(),
                amount: total - amount,
                id: Uuid::new_v4().to_string(),
            });
        }

        // Step 4: Add new UTXOs
        for output in &outputs {
            self.utxos.insert(output.id.clone(), output.clone());
        }

        Some(Transaction { inputs, outputs })
    }

    fn print_utxos(&self) {
        println!("--- Current UTXOs ---");
        for utxo in self.utxos.values() {
            println!("{:?}", utxo);
        }
    }
}

fn main() {
    let mut chain = Blockchain::new();

    // Genesis: Alice gets 100 coins
    chain.create_genesis("Alice", 100);
    chain.print_utxos();

    // Alice sends 60 to Bob
    chain.create_transaction("Alice", "Bob", 60);
    chain.print_utxos();

    // Bob sends 30 to Carol
    chain.create_transaction("Bob", "Carol", 30);
    chain.print_utxos();
}
