
fn main() {
    let mut items = vec![Some(10), Some(20), None, Some(30)];

    
    // ✅ while let loop
    while let Some(option) = items.pop() {
        // ✅ match inside while let
        match option {
            Some(val) if val > 15 => println!("Big number: {}", val),  // Pattern Guard
            Some(val) => println!("Number: {}", val),
            None => println!("Empty slot."),
        }
    }
}
