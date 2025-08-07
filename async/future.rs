
use std::future::Future;

fn custom_future() -> impl Future<Output = i32> {
    async { 42 }
}

#[tokio::main]
async fn main() {
    let value = custom_future().await;
    println!("Got: {}", value);
}
