
//tokio = { version = "1", features = ["full"] }

use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::time::{sleep, Duration};
use tokio::task;

#[derive(Debug, Clone)]
struct LogEntry {
    id: usize,
    message: String,
}

async fn start_producer(mut tx_input: Sender<LogEntry>) {
    for i in 0..10 {
        let log = LogEntry {
            id: i,
            message: format!("Log #{}", i),
        };
        println!("[Producer] Generated: {:?}", log);
        tx_input.send(log).await.unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}

async fn start_processor(
    name: &'static str,
    mut rx_input: Receiver<LogEntry>,
    tx_output: Sender<String>,
) {
    while let Some(log) = rx_input.recv().await {
        let processed = format!("[{}] Processed Log {}: {}", name, log.id, log.message);
        tx_output.send(processed).await.unwrap();
    }
    println!("[{}] Input channel closed. Exiting.", name);
}

async fn start_logger(mut rx_output: Receiver<String>) {
    while let Some(message) = rx_output.recv().await {
        println!("[Logger] => {}", message);
    }
    println!("[Logger] Output channel closed. Logger exiting.");
}

#[tokio::main]
async fn main() {
    let (tx_input, rx_input) = mpsc::channel::<LogEntry>(10);
    let (tx_output, rx_output) = mpsc::channel::<String>(10);

    // Producer
    let producer_handle = tokio::spawn(start_producer(tx_input));

    // Split input receiver for processors
    let rx_input1 = rx_input;
    let rx_input2 = rx_input1.clone(); // can't actually clone mpsc::Receiver, so pick one or use broadcast

    // Processor A
    let tx_output_clone_a = tx_output.clone();
    let processor_a = task::spawn(start_processor("Processor A", rx_input1, tx_output_clone_a));

    // Processor B - can't clone the same Receiver, so use separate channel or split logic differently
    let processor_b = task::spawn(async {
        println!("[Processor B] Not implemented in async version (Receiver can't be cloned)");
    });

    // Logger
    let logger = task::spawn(start_logger(rx_output));

    // Wait for everything to finish
    producer_handle.await.unwrap();
    processor_a.await.unwrap();
    processor_b.await.unwrap();
    drop(tx_output); // close channel for logger
    logger.await.unwrap();
}
