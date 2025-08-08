
mod processor;
mod collector;
mod utils;

use processor::process_logs;
use collector::run_collector;
use utils::init_logger;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tokio::signal;

#[tokio::main]
async fn main() {
    init_logger();

    // Create cancellation token
    let cancel_token = CancellationToken::new();
    let child_token = cancel_token.child_token();

    // Create channel
    let (tx, rx) = mpsc::channel::<String>(100);

    // Spawn log processor
    let processor_handle = tokio::spawn(process_logs(child_token.clone(), rx));

    // Spawn collector
    let collector_handle = tokio::spawn(run_collector(tx));

    // Listen for Ctrl+C
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Ctrl+C received, shutting down...");
            cancel_token.cancel();
        }
    }

    // Wait for both tasks to finish
    let _ = processor_handle.await;
    let _ = collector_handle.await;
}
