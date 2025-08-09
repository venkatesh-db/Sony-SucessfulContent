use tokio::time::{sleep, Duration};
use futures::{Stream, StreamExt};
use pin_utils::pin_mut;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::join;
use tokio::select;

// -----------------------------
// 1. Custom Stream Implementation
// -----------------------------
struct Counter {
    count: usize,
    max: usize,
}

impl Stream for Counter {
    type Item = usize;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if self.count < self.max {
            self.count += 1;
            Poll::Ready(Some(self.count))
        } else {
            Poll::Ready(None)
        }
    }
}

// -----------------------------
// 2. Async functions for join!/select!
// -----------------------------
async fn async_task_a() {
    for i in 1..=3 {
        println!("Task A - step {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

async fn async_task_b() {
    for i in 1..=3 {
        println!("Task B - step {}", i);
        sleep(Duration::from_millis(700)).await;
    }
}

// -----------------------------
// 3. Main async runtime
// -----------------------------
#[tokio::main]
async fn main() {
    println!("--- Pinning + Custom Stream ---");
    let counter = Counter { count: 0, max: 5 };
    pin_mut!(counter); // Pin the stream in memory

    while let Some(val) = counter.next().await {
        println!("Counter produced: {}", val);
    }

    println!("\n--- Using join! ---");
    join!(async_task_a(), async_task_b());

    println!("\n--- Using select! ---");
    let mut t1 = async {
        sleep(Duration::from_secs(1)).await;
        "First task done"
    };
    let mut t2 = async {
        sleep(Duration::from_secs(2)).await;
        "Second task done"
    };

    tokio::pin!(t1);
    tokio::pin!(t2);

    select! {
        res = &mut t1 => println!("{}", res),
        res = &mut t2 => println!("{}", res),
    }
}
