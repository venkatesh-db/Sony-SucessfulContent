
use std::rc::Rc;

fn main() {

    let data = Rc::new("Rust is awesome!".to_string());

     let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    println!("a: {}", a);
    println!("b: {}", b);
    println!("Reference Count: {}", Rc::strong_count(&data)); // 3
}
