
trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let pet: Box<dyn Animal> = Box::new(Dog);
    pet.speak();  // Dynamic dispatch
}
