
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context, Waker};

struct Done;

impl Future for Done {

    type Output = &'static str;
    
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("done!")
    }
}
