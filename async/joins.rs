
use tokio::{join, time::{sleep, Duration}};

async fn task1() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 1 done");
}

async fn task2() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 done");
}

#[tokio::main]
async fn main() {
    join!(task1(), task2());
}
