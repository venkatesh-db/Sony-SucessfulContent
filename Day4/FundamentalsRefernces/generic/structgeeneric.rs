
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(val: T) -> Self {
        Self { value: val }
    }
    fn get(&self) -> &T {
        &self.value
    }
}
fn main() {
    let int_container = Container::new(10);
    let str_container = Container::new("Rust");
    println!("Int: {}", int_container.get());
    println!("Str: {}", str_container.get());
}
