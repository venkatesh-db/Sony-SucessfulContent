
mod api;
mod pool;
mod metrics;

use axum::{routing::get, Router};
use metrics::metrics_handler;
use pool::start_pool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Start the worker pool in background
    tokio::spawn(start_pool());

    // Setup metrics route
    let app = Router::new().route("/metrics", get(metrics_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Serving metrics on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}