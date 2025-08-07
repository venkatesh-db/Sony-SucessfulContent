
use std::fmt;

#[derive(Debug)]
pub enum UpiError {
    InsufficientFunds,
    UserNotFound,
    InvalidAmount,
    InternalError(String),
}

impl fmt::Display for UpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpiError::InsufficientFunds => write!(f, "Insufficient funds."),
            UpiError::UserNotFound => write!(f, "User not found."),
            UpiError::InvalidAmount => write!(f, "Invalid amount."),
            UpiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

pub trait ErrorLoggable {
    fn log(&self);
}

impl ErrorLoggable for UpiError {
    fn log(&self) {
        eprintln!("❌ ERROR: {}", self);
    }
}
