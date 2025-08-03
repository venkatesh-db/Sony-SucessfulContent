

/* 
fn smile<T: > (){

}
*/

fn print_value<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

/* 
fn print_value(value:i32){ _z3print_valuei

}

fn print_value(value:f32){ _z3print_valuef
    
}
*/

fn main() {

    print_value(42);        // i32
    print_value(3.14);      // f64
    print_value("hello");   // &str

}
