use std::{fs::OpenOptions, io::Write, sync::Arc};
use chrono::Local;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::task;
use std::fmt;

// Constants
const LOG_FILE: &str = "logs/app.log";

// Logger Trait
trait Logger {
    fn log(&self, level: LogLevel, message: String);
}

// Log Levels
enum LogLevel {
    INFO,
    WARN,
    ERROR,
}

impl fmt::Display for LogLevel {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::ERROR => write!(f, "ERROR"),
        }

    }
}

// Logger Struct
struct AsyncLogger {
    sender: Arc<Mutex<Sender<String>>>,
}

impl AsyncLogger {
    
    async fn new() -> Arc<Self> {

       // let (tx, mut rx) = mpsc::channel::<String>(1000);

       let (tx, mut rx)=mpsc::channel::<String>(1000);

       std::fs::create_dir_all("logs").expect("happy to see error");

        // Ensure "logs/" folder exists
       // std::fs::create_dir_all("logs").expect("Failed to create logs directory");

        // Spawn background task
        task::spawn(async move {
            
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(LOG_FILE)
                .expect("Cannot open log file.");

            while let Some(log_message) = rx.recv().await {
                if let Err(e) = writeln!(file, "{}", log_message) {
                    eprintln!("Logger Write Error: {}", e);
                }
            }
        });

        Arc::new(Self {
            sender: Arc::new(Mutex::new(tx)),
        })
    }
}

impl Logger for AsyncLogger {
    fn log(&self, level: LogLevel, message: String) {

        let tx_clone = Arc::clone(&self.sender);
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        let log_line = format!("[{}] [{}] {}", timestamp, level, message);

        // Non-blocking send
        tokio::spawn(async move {
            let mut sender = tx_clone.lock().await;
            if sender.send(log_line).await.is_err() {
                eprintln!("Logger Channel Full or Closed.");
            }
        });
    }
}

// Macro for clean usage
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

#[tokio::main]
async fn main() {
    // Initialize Logger
    let logger = AsyncLogger::new().await;

    // Simulate Logs
    for i in 0..100 {
        app_log!(logger, INFO, format!("Processing transaction: UPI_TXN_{}", i));
    }

    app_log!(logger, WARN, "Transaction lag detected.");
    app_log!(logger, ERROR, "Transaction failed due to timeout.");

    app_log!(logger, INFO, "smiling morning with idly vdada.");

    println!("Logs written asynchronously to: {}", LOG_FILE);
}
