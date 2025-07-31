
fn main() {
    let number = 42;

    match number {
        n if n % 2 == 0 => println!("Even number: {}", n),  // Pattern Guard
        n if n % 2 != 0 => println!("Odd number: {}", n),
        _ => println!("Unknown number."),
    }
}
