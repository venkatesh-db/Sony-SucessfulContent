
use std::pin::Pin;

fn main() {
    let x = Box::pin(5);
    println!("Pinned value = {}", *x);
}

