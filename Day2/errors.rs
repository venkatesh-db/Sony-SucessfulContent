
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Venkatesh".to_string()),
        2 => Some("Ravi".to_string()),
        _ => None,
    }
}

fn greet_user(id: u32) -> Option<String> {
    let name = find_user(id)?;  // ? operator auto-returns None if find_user returns None
    Some(format!("Hello, {}!", name))
}

fn main() {
    let greeting = greet_user(1).unwrap_or("User not found.".to_string());
    println!("{}", greeting);

    let greeting2 = greet_user(99).unwrap_or("User not found.".to_string());
    println!("{}", greeting2);
}
