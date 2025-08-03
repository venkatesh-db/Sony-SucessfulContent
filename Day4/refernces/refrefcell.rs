
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared_data = Rc::new(RefCell::new(10));

    let a = Rc::clone(&shared_data);
    let b = Rc::clone(&shared_data);

    *a.borrow_mut() += 5;
    println!("A: {}", a.borrow());  // 15

    *b.borrow_mut() += 5;
    println!("B: {}", b.borrow());  // 20
}
