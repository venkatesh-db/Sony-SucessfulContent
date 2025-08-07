
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("Got: {}", msg);
    }
}
