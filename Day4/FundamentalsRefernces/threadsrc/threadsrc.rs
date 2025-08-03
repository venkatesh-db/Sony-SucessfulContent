
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));

    let v1 = Rc::clone(&shared_vec);
    let v2 = Rc::clone(&shared_vec);

    v1.borrow_mut().push(4);
    v2.borrow_mut().push(5);

    println!("{:?}", shared_vec.borrow()); // Output: [1, 2, 3, 4, 5]
}
