
use crate::runtime::{ICIC, Paymentreverse};
use crate::oops::{Payment};


mod jolly;
mod oops;
mod runtime;

// import jolly
// from jolly impoort ICIC

fn main() {
    
    println!("Hello, world!");
    jolly::Smiles();
    let naseer=oops::Payment{ arr:[1,2,3,4,5]};
    naseer.Transaction();
    
   let retss:ICIC= runtime::ICIC::new(2,true);
   retss.Retry(2);
}
