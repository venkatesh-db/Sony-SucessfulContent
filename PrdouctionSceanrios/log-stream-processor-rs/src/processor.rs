
use tokio::sync::mpsc::{Receiver};
use tokio_util::sync::CancellationToken;

pub async fn process_logs(cancel: CancellationToken, mut rx: Receiver<String>) {
    while !cancel.is_cancelled() {
        tokio::select! {
            Some(log) = rx.recv() => {
                // Process log
                tokio::spawn(async move {
                    // Lifetime issue example
                    do_something_with_log(&log).await; // ❌ borrow error
                });
            }
            _ = cancel.cancelled() => {
                println!("Processor shutting down");
                break;
            }
        }
    }
}

async fn do_something_with_log(log: &str) {
    println!("Processed: {}", log);
}
