
use std::fmt::Display;

// 1. Generic Struct
struct Boxed<T> {
    item: T,
}

// 2. Trait Definition
trait Show {
    fn show(&self);
}

// 3. Generic Trait Implementation (Bounded)
impl<T: Display> Show for Boxed<T> {
    fn show(&self) {
        println!("Item: {}", self.item);
    }
}

fn main() {
    let b = Boxed { item: 42 };
    b.show();  // Works because i32 implements Display
}
