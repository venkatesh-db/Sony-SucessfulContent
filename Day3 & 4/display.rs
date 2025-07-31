
//#[derive(Debug)]

struct User {
    name: String,
}

impl std::fmt::Display for User {
   fn fmt(&self,f:&mut std::fmt::Formatter ) ->std::fmt::Result {
           write!( f,"smiling rust coders {}",self.name) 
   }
}

/* 
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!(" smiling");
        write!(f, "User: {}", self.name)
    }
}
*/




fn main() {
    let user = User { name: "Alice".into() };

    println!("{}", user);     // Calls Display
 //  println!("{:?}", user);    // Calls Debug
}
