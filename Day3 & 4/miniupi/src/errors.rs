use std::fmt;

#[derive(Debug)]
pub enum UpiError{
   InsufficientFunds,
   UsernotFound,
   InvalidAmount,
   InternalError(String),
}

impl fmt::Display for UpiError{

  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self{

            UpiError::InsufficientFunds =>write!(f,"InsuffiecientFunds"),
            UpiError::UsernotFound =>write!(f,"UsernotFound"),
             UpiError::InvalidAmount => write!(f,"InvalidAmount"),
             UpiError::InternalError(msg)=> write!(f,"internal error{}",msg),
        }

  }
}

pub trait ErrorLoggable{
   fn log(&self);
}

impl ErrorLoggable for UpiError {

  fn log(&self)
  {
    eprintln!("error {}",self);
  }
}
