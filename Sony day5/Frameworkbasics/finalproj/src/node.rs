
use axum::{
    routing::get,
    Router,
    response::IntoResponse,
};

use std::net::SocketAddr;

pub async fn run_node() {
    // Build the app with some routes
    let app = Router::new()
        .route("/", get(root))
        .route("/blocks", get(get_blocks));

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    "Welcome to MiniChain Node API"
}

async fn get_blocks() -> impl IntoResponse {
    // In a real app, you'd fetch from chain state here
    axum::Json(vec!["Genesis Block", "Block 2", "Block 3"])
}
