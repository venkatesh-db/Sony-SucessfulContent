
use std::time::Duration;
use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::task;
use tokio::time::sleep;

const WORKER_COUNT: usize = 4;
const JOB_COUNT: usize = 20;

#[derive(Debug)]
struct Job {
    id: usize,
}

async fn worker(id: usize, job: Job) {
    println!("Worker-{} processing Job-{}", id, job.id);

    // Simulate work
    sleep(Duration::from_millis(500)).await;

    println!("Worker-{} finished Job-{}", id, job.id);
}

async fn dispatcher(mut rx: Receiver<Job>) {

   println!("Dispatcher entry");

    while let Some(job) = rx.recv().await {
    
        // Dispatch job to first-available worker
        println!("friday dispatcher.");
        
        task::spawn(worker(job.id % WORKER_COUNT + 1, job));
        
         println!("friday afternoon dispatcher.");
    }
    println!("Dispatcher: no more jobs.");
}

#[tokio::main]
async fn main() {

    let (tx, rx) = mpsc::channel::<Job>(JOB_COUNT);
    // Spawn the central dispatcher task
    task::spawn(dispatcher(rx));

    println!("before loops");

    // Submit jobs to the queue
    for id in 0..JOB_COUNT {
            println!("for  loops entry ");
        let job = Job { id };
        if tx.send(job).await.is_err() {
            eprintln!("Failed to send job.");
        }
    }

     println!("drop  channel ");
    drop(tx);  // Close the sender so dispatcher can finish after processing

    println!("All jobs dispatched.");
    sleep(Duration::from_secs(7)).await;  // Wait for workers to finish
}
