
use std::pin::Pin;
use std::future::Future;

async fn compute() -> i32 {
    10
}

fn use_pin(f: impl Future<Output = i32> + Unpin) {
    let mut f = Box::pin(f); // Safe due to Unpin
}
