
// Define a Trait
trait Greet {
    fn say_hello(&self);
}

// Struct implementing the Trait
struct Person {
    name: String,
}

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let p = Person {
        name: "Venkatesh".to_string(),
    };

    p.say_hello();  // Call trait method
}
