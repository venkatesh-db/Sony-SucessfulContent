

use tokio::sync::mpsc;

async fn producer(tx: mpsc::Sender<String>) {
    tx.send("Hello".to_string()).await.unwrap();
}

async fn consumer(mut rx: mpsc::Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);

    tokio::spawn(producer(tx));
    consumer(rx).await;
}
