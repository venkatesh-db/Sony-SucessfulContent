
use rand::Rng;
use std::error::Error;
use tokio::time::{sleep, Duration};

pub async fn make_api_call(payload: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Simulate network delay
    let delay = rand::thread_rng().gen_range(10..50);
    sleep(Duration::from_millis(delay)).await;

    // Mock success/failure
    if rand::random::<f32>() < 0.95 {
        Ok(payload)
    } else {
        Err("Mock API failure".into())
    }
}
