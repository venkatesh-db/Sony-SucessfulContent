
fn main() {
    let data = ["10", "abc", "42"];

    for item in data {
        let result = item.parse::<i32>();

        match result {
            Ok(num) => println!("Parsed: {}", num),
            Err(_) => println!("Failed to parse: {}", item),
        }
    }
}
