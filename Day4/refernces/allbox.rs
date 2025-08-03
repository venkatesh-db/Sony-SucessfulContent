
use std::sync::Arc;

fn main() {
    let b = Box::new(42);
    println!("Boxed: {}", b);

    let rc = Rc::new(100);
    let rc2 = Rc::clone(&rc);

    let arc = Arc::new("Hello");
    let arc2 = Arc::clone(&arc);
}
