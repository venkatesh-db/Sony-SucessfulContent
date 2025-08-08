
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;

static SUCCESS_COUNT: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

pub fn increment_counter() {
    SUCCESS_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn print_metrics() {
    println!();
    println!("================ METRICS ================");
    println!("✅ Successful Calls: {}", SUCCESS_COUNT.load(Ordering::Relaxed));
    println!("=========================================");
}
