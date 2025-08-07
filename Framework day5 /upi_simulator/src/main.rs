
use std::io::{self, Write};

// User Account
#[derive(Clone)]
struct User {
    name: String,
    balance: f64,
    transactions: Vec<String>,
}

// UPI Simulator
struct UpiApp {
    users: Vec<User>,
}

impl UpiApp {
    fn new() -> Self {
        Self { users: Vec::new() }
    }

    fn create_user(&mut self, name: &str, balance: f64) {
        let user = User {
            name: name.to_string(),
            balance,
            transactions: vec![format!("Account created with ₹{:.2}", balance)],
        };
        self.users.push(user);
        println!("User {} created successfully.", name);
    }

    fn find_user_index(&self, name: &str) -> Option<usize> {
        for (index, user) in self.users.iter().enumerate() {
            if user.name == name {
                return Some(index);
            }
        }
        None
    }

    fn transfer(&mut self, from: &str, to: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Invalid transfer amount.".to_string());
        }

        if from == to {
            return Err("Cannot transfer to self.".to_string());
        }

        let from_index = self.find_user_index(from);
        let to_index = self.find_user_index(to);

        if from_index.is_none() || to_index.is_none() {
            return Err("Invalid sender or receiver.".to_string());
        }

        let from_idx = from_index.unwrap();
        let to_idx = to_index.unwrap();

        if self.users[from_idx].balance < amount {
            return Err("Insufficient funds.".to_string());
        }

        self.users[from_idx].balance -= amount;
        self.users[to_idx].balance += amount;

        self.users[from_idx]
            .transactions
            .push(format!("Transferred ₹{:.2} to {}", amount, to));
        self.users[to_idx]
            .transactions
            .push(format!("Received ₹{:.2} from {}", amount, from));

        println!(
            "[LOG] {} transferred ₹{:.2} to {}",
            from, amount, to
        );

        Ok(())
    }

    fn statement(&self, user_name: &str) -> Result<(), String> {
        let index = self.find_user_index(user_name);
        if index.is_none() {
            return Err("User not found.".to_string());
        }

        let user = &self.users[index.unwrap()];
        println!("--- Statement for {} ---", user_name);
        println!("Balance: ₹{:.2}", user.balance);
        for txn in &user.transactions {
            println!("{}", txn);
        }

        Ok(())
    }
}

fn main() {
    let mut upi_app = UpiApp::new();

    loop {
        println!("\n1. Create User\n2. Transfer\n3. Statement\n4. Exit");
        print!("Choose: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("User Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                print!("Initial Balance ₹: ");
                io::stdout().flush().unwrap();
                let mut bal = String::new();
                io::stdin().read_line(&mut bal).unwrap();
                let balance: f64 = bal.trim().parse().unwrap_or(0.0);

                upi_app.create_user(name.trim(), balance);
            }
            "2" => {
                print!("From User: ");
                io::stdout().flush().unwrap();
                let mut from = String::new();
                io::stdin().read_line(&mut from).unwrap();

                print!("To User: ");
                io::stdout().flush().unwrap();
                let mut to = String::new();
                io::stdin().read_line(&mut to).unwrap();

                print!("Amount ₹: ");
                io::stdout().flush().unwrap();
                let mut amt = String::new();
                io::stdin().read_line(&mut amt).unwrap();
                let amount: f64 = amt.trim().parse().unwrap_or(0.0);

                match upi_app.transfer(from.trim(), to.trim(), amount) {
                    Ok(_) => println!("Transfer Successful."),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                print!("User Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                if let Err(e) = upi_app.statement(name.trim()) {
                    println!("Error: {}", e);
                }
            }
            "4" => break,
            _ => println!("Invalid choice."),
        }
    }
}
