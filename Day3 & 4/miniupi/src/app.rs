use std::collections::HashMap;

use crate::models::{Transaction, User};
use crate::errors::{UpiError, ErrorLoggable};
use crate::logger::{Logger, ConsoleLogger};

pub struct UpiApp {
    pub users: HashMap<String, User>,
    pub logger: Box<dyn Logger>,
}

impl UpiApp {
    pub fn new() -> Self {
        UpiApp {
            users: HashMap::new(),
            logger: Box::new(ConsoleLogger),
        }
    }

    pub fn add_user(&mut self, name: &str, balance: f64) {
        let user = User::new(name, balance);
        self.users.insert(name.to_string(), user);
    }

    fn get_user_mut(&mut self, name: &str) -> Result<&mut User, UpiError> {
        self.users.get_mut(name).ok_or(UpiError::UsernotFound)
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: f64) -> Result<(), UpiError> {
       
        if amount <= 0.0 {
            return Err(UpiError::InvalidAmount);
        }

        let from_user = self.get_user_mut(from)?;
        
        if from_user.balance < amount {
            return Err(UpiError::InsufficientFunds);
        }

        let from_id = from_user.id;
        from_user.balance -= amount;

        let to_user = self.get_user_mut(to)?;
        let to_id = to_user.id;
        to_user.balance += amount;

        let txn = Transaction::new(from_id, to_id, amount);
        self.logger.log_transaction(&txn);

        Ok(())
    }

    pub fn balance(&self, name: &str) -> Result<f64, UpiError> {
        self.users.get(name)
            .map(|u| u.balance)
            .ok_or(UpiError::UsernotFound)
    }
}
