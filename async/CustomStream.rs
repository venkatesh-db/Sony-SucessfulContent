
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;

struct Counter {
    count: u8,
}

impl Stream for Counter {
    type Item = u8;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if self.count < 5 {
            let val = self.count;
            self.count += 1;
            Poll::Ready(Some(val))
        } else {
            Poll::Ready(None)
        }
    }
}
