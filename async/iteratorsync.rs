
use futures::stream::Stream;
use futures::stream;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let mut s = stream::iter(vec![1, 2, 3]);

    while let Some(val) = s.next().await {
        println!("Got {}", val);
    }
}
