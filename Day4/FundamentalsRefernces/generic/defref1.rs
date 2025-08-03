
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = MyBox(String::from("hello"));

    // Automatic deref coercion
    println!("Length: {}", b.len()); // works like (*b).len()
}
