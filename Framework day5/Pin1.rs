use std::pin::Pin;

struct Laddu {
    name: String,
}

fn main() {
    let laddu = Laddu { name: "Sweet".to_string() };
    let pinned = Pin::new(&laddu);
    println!("Laddu = {}", pinned.name);
}
