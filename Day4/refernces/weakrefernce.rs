
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let a = Rc::new(RefCell::new(5));
    let b: Weak<_> = Rc::downgrade(&a);

    println!("Strong Count: {}", Rc::strong_count(&a));  // 1
    println!("Weak Count: {}", Rc::weak_count(&a));      // 1

    if let Some(strong) = b.upgrade() {
        println!("Value: {}", strong.borrow());
    }
}
