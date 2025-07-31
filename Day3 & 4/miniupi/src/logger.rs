
use crate::models::Transaction;


pub trait Logger{
    fn log_transaction(&self, txn:&Transaction );
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger{

 fn log_transaction(&self, txn:&Transaction ){

    println!(" {} {} {} ",txn.id,txn.amount,txn.from );
 }
  

}