
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Set up the Axum router
    let app = Router::new()
        .route("/", get(root))
        .route("/echo", post(echo));

    // Bind to an address using Tokio TcpListener
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running at http://127.0.0.1:3000");

    // Serve the app using Axum's built-in server
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, Venkatesh! 👋"
}

#[derive(Deserialize, Serialize)]
struct Message {
    msg: String,
}

async fn echo(Json(payload): Json<Message>) -> Json<Message> {
    Json(payload)
}
