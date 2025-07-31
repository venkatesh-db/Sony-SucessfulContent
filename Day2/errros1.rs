
// Custom Error Type
#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    NegativeAmount,
}

// Bank Account Struct
struct BankAccount {
    balance: f64,
}

impl BankAccount {
    // Method returning Result with ? operator
    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount < 0.0 {
            return Err(BankError::NegativeAmount);
        }

        if self.balance < amount {
            return Err(BankError::InsufficientFunds);
        }

        self.balance -= amount;
        Ok(())
    }

    fn check_balance(&self) -> f64 {
        self.balance
    }
}

fn main() -> Result<(), BankError> {
    let mut account = BankAccount { balance: 500.0 };

    account.withdraw(100.0)?;  // uses ? operator
    println!("Balance after withdrawal: ₹{}", account.check_balance());

    account.withdraw(999.0)?;  // will trigger custom error and exit early

    println!("This line won't print if error occurred.");
    Ok(())
}
