
// Generic Swap Function
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

// Simple Enum with Result usage
enum Status {
    Active,
    Inactive,
}

fn status_message(status: Status) -> Result<&'static str, &'static str> {
    match status {
        Status::Active => Ok("User is active."),
        Status::Inactive => Err("User is inactive."),
    }
}

fn main() {
    // Using swap()
    let mut x = 10;
    let mut y = 20;
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);

    // Using Status Enum with Result
    let msg = status_message(Status::Inactive).unwrap_or("Unknown status.");
    println!("{}", msg);
}
