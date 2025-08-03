
fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = Box::new(String::from("Venkatesh"));
    greet(&x); // works due to Deref<Target=String> + Deref<Target=str>
}
