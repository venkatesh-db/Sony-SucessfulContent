/* 
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
async-trait = "0.1"
*/

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    say_hello().await;
}

async fn say_hello() {
    sleep(Duration::from_secs(1)).await;
    println!("Hello after 1 second!");
}
