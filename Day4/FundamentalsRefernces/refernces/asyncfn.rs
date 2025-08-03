
//tokio = { version = "1", features = ["full"] }

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        tx.send("Hello from task!").await.unwrap();
    });

    if let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}

