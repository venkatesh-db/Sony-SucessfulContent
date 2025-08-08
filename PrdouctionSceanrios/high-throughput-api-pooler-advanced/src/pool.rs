
use crate::api::call_api;
use crate::metrics::{ACTIVE_WORKERS, TOTAL_CALLS};
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};

const MAX_WORKERS: usize = 10;
const QUEUE_SIZE: usize = 100;

pub async fn start_pool() {
    let (tx, mut rx) = mpsc::channel(QUEUE_SIZE);
    let semaphore = Arc::new(Semaphore::new(MAX_WORKERS));

    // Load generator
    tokio::spawn({
        let tx = tx.clone();
        async move {
            loop {
                if tx.send(()).await.is_err() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    });

    // Worker consumers
    while let Some(_) = rx.recv().await {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        ACTIVE_WORKERS.inc();
        TOTAL_CALLS.inc();

        tokio::spawn(async move {
            let _ = call_api("https://httpbin.org/get").await;
            drop(permit);
            ACTIVE_WORKERS.dec();
        });
    }
}
