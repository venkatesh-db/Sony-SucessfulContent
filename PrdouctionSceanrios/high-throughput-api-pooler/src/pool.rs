
use crate::api::make_api_call;
use crate::metrics::increment_counter;
use tokio::task;

pub fn start_pool(payload: String) {
    task::spawn(async move {
        match make_api_call(payload.clone()).await {
            Ok(res) => {
                println!("Processed: {}", res);
                increment_counter();
            }
            Err(err) => {
                eprintln!("Failed: {} | Payload: {}", err, payload);
            }
        }
    });
}
