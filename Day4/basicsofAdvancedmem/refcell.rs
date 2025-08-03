
use std::cell::RefCell;

fn main() {
    let val = RefCell::new(42);

    // Borrow mutably
    *val.borrow_mut() += 1;

    // Borrow immutably
    println!("Value: {}", val.borrow());
}