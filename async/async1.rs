
use tokio::time::{sleep, Duration};

async fn task_a() {
    println!("Task A started");
    sleep(Duration::from_secs(2)).await;
    println!("Task A completed");
}

#[tokio::main]
async fn main() {
    println!("Main started");
    task_a().await; // waits without blocking
    println!("Main ended");
}
