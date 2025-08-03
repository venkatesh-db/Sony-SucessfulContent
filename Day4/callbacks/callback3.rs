
fn call_me(cb: Box<dyn Fn(i32) -> i32>) {
    let out = cb(4);
    println!("Output: {}", out);
}

fn main() {
    let my_cb = Box::new(|x| x + 7);
    call_me(my_cb); // Output: 11
}
