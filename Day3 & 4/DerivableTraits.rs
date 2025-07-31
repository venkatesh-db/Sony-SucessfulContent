
//#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

fn main() {

    let u1 = User { name: "Alice".into(), age: 30 };
 //   let u2 = u1.clone();

    println!("{}", u1);

    if u1 == u2 {
        println!("Users are equal");
    }
}
