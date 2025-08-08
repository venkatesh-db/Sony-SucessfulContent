mod pool;
mod api;
mod metrics;

use pool::start_pool;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting high-throughput API pooler...");

    // Simulate incoming tasks
    for i in 0..100 {
        let payload = format!("Hello from service A - {}", i);
        start_pool(payload);
        sleep(Duration::from_millis(10)).await;
    }

    // Let tasks finish
    sleep(Duration::from_secs(5)).await;

    metrics::print_metrics();
}
