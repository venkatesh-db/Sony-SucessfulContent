
use tracing::{info, error};
use tracing_subscriber;

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();
}
