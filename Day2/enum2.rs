
enum MaybeNumber {
    Some(i32),
    None,
}

fn main() {
    let num = MaybeNumber::Some(42);

    match num {
        MaybeNumber::Some(val) => println!("Value: {}", val),
        MaybeNumber::None => println!("No value found."),
    }
}
