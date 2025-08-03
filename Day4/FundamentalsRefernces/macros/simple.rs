
#[derive(Debug)]
enum LogLevel {
    INFO,
    WARN,
    ERROR,
}

struct Logger;

impl Logger {
    fn log(&self, level: LogLevel, message: String) {
        println!("[{:?}] {}", level, message);
    }
}

macro_rules! app_log {
    ($logger:expr, INFO, $msg:expr) => {
        $logger.log(LogLevel::INFO, $msg.to_string());
    };
    ($logger:expr, WARN, $msg:expr) => {
        $logger.log(LogLevel::WARN, $msg.to_string());
    };
    ($logger:expr, ERROR, $msg:expr) => {
        $logger.log(LogLevel::ERROR, $msg.to_string());
    };
}

fn main() {
    let logger = Logger;

    app_log!(logger, INFO, "Server started");
    app_log!(logger, WARN, "Low disk space");
    app_log!(logger, ERROR, "Service crashed");
}
