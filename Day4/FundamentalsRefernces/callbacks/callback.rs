
fn process_number(x: i32, callback: fn(i32) -> i32) -> i32 {
    callback(x) * 2
}

fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    let result = process_number(5, square);
    println!("Result: {}", result);  // Result: 50
}
