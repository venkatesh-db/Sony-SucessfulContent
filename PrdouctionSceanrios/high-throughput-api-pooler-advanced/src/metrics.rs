

use axum::{response::IntoResponse, Json};
use lazy_static::lazy_static;
use prometheus::{Encoder, IntCounter, IntGauge, TextEncoder};
use std::sync::Mutex;

lazy_static! {
    pub static ref TOTAL_CALLS: IntCounter = IntCounter::new("total_calls", "Total API Calls").unwrap();
    pub static ref ACTIVE_WORKERS: IntGauge = IntGauge::new("active_workers", "Active Worker Count").unwrap();
    static ref REGISTRY: Mutex<()> = {
        prometheus::default_registry().register(Box::new(TOTAL_CALLS.clone())).ok();
        prometheus::default_registry().register(Box::new(ACTIVE_WORKERS.clone())).ok();
        Mutex::new(())
    };
}

pub async fn metrics_handler() -> impl IntoResponse {
  drop(REGISTRY.lock()); // if you just want to touch the lock momentarily
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Json(String::from_utf8(buffer).unwrap())
}
