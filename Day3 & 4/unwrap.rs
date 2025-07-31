
fn main() {

    let some_value = Some(42);
    let result = some_value.unwrap();   // extracts 42
    println!("The value is: {}", result);

}


fn main() {
    
    let result: Result<i32, &str> = Ok(10);
    let value = result.unwrap();   // extracts 10

    println!("Value is: {}", value);
}
