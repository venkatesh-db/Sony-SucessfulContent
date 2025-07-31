
fn main() {
    // ✅ Option + unwrap
    let names = ["Venkatesh", "Ravi", "John"];
    let name = names.get(9).unwrap();  // Will panic if index invalid
    println!("Found name: {}", name);

    // ✅ panic! example
    let value = 10;
    if value > 5 {
        panic!("Value exceeded limit!");  // Force panic
    }

    println!("This line won't print due to panic.");
}
