
// Define a Trait
trait Logger {

    fn log(&self, message: &str);

}
// Implement the Trait for ConsoleLogger
struct ConsoleLogger;

impl Logger for ConsoleLogger {

    fn log(&self, message: &str) {
        println!("Console: {}", message);
    }

}

fn poly( refvr:&dyn Logger ){

        refvr.log("venkatesh");
}

fn main(){

       let console = ConsoleLogger;
       console.log("hello");
       poly(&console);
       
}