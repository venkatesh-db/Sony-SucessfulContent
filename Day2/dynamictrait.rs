
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

// Implement the Trait for FileLogger
struct FileLogger;

impl Logger for FileLogger {

    fn log(&self, message: &str) {
        println!("FileLogger (fake): {}", message);  // Simulating file write
    }
}

// Function accepting trait object
fn send_log(logger: &dyn Logger, message: &str) {

    logger.log(message);
}

fn main() {
    
    let console = ConsoleLogger;
    let file = FileLogger;

    // ✅ Dynamic dispatch via trait object
    send_log(&console, "Hello via Console Logger");
    send_log(&file, "Hello via File Logger");

    // ✅ Storing multiple types as trait objects in a vector
    let loggers: Vec<Box<dyn Logger>> = vec![
        Box::new(ConsoleLogger),
        Box::new(FileLogger),
    ];

    for logger in loggers {
        logger.log("Dynamic Trait Object Logging!");
    }
}
