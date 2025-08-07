
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use std::time::Duration;
use tokio::time::Sleep;

struct MyDelay {
    sleep: Pin<Box<Sleep>>,
}

impl Future for MyDelay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.sleep.as_mut().poll(cx).is_ready() {
            Poll::Ready("Done sleeping")
        } else {
            Poll::Pending
        }
    }
}
