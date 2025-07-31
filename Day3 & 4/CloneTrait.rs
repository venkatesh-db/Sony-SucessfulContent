
#[derive(Debug)]
struct User {
    name: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User { name: self.name.clone() }
    }
}

fn main() {
    let u1 = User { name: "Alice".into() };
    let u2 = u1.clone();
    println!("{:?}", u2);
}
