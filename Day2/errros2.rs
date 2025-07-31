
#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    NegativeAmount,
}

impl std::fmt::Display for BankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankError::InsufficientFunds => write!(f, "❌ Error: Insufficient funds."),
            BankError::NegativeAmount => write!(f, "❌ Error: Cannot withdraw negative amount."),
        }
    }
}

struct BankAccount {
    balance: f64,
}

impl BankAccount {
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

fn main() {
    let mut account = BankAccount { balance: 500.0 };

    match account.withdraw(100.0) {
        Ok(_) => println!("✅ Withdrawal successful. Balance: ₹{}", account.check_balance()),
        Err(e) => println!("{}", e),
    }

    match account.withdraw(999.0) {
        Ok(_) => println!("✅ Withdrawal successful. Balance: ₹{}", account.check_balance()),
        Err(e) => println!("{}", e),  // ❌ This prints the error message
    }
}
