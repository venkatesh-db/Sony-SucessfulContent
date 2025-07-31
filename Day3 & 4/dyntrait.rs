
trait Logger {
    fn log(&self, message: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Console: {}", message);
    }
}

fn use_logger(logger: &dyn Logger) {
    logger.log("Hello from dynamic dispatch!");
}

fn main() {
    let logger = ConsoleLogger;
    use_logger(&logger);
}
