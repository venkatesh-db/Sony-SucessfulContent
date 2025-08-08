
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;

pub async fn run_collector(sender: Sender<String>) {
    let listener = TcpListener::bind("0.0.0.0:9999").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let tx = sender.clone();

        tokio::spawn(async move {
            use tokio::io::{AsyncBufReadExt, BufReader};
            let mut reader = BufReader::new(socket);
            let mut line = String::new();

            while reader.read_line(&mut line).await.unwrap() > 0 {
                let heap_log = Arc::new(line.clone()); // heap allocation here
                if let Err(_) = tx.send((*heap_log).clone()).await {
                    eprintln!("Send failed");
                }
                line.clear();
            }
        });
    }
}
