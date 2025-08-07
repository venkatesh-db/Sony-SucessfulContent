
use tokio_stream::{StreamExt, wrappers::IntervalStream};
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {

    let interval = interval(Duration::from_secs(1));
    
    let mut stream = IntervalStream::new(interval);

    while let Some(_) = stream.next().await {
        println!("Tick");
    }
}

