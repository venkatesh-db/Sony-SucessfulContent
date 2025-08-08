
use axum::{Json, Router, routing::{get, post}, extract::Query};
use std::net::SocketAddr;
use crate::chain::{BLOCKCHAIN};
use crate::chain::Block;
use serde::Deserialize;

#[derive(Deserialize)]
struct New { data: String }

async fn get_chain() -> Json<Vec<Block>> {
    let bc = BLOCKCHAIN.lock().unwrap();
    Json(bc.chain.clone())
}

async fn add_block(Query(q): Query<New>) -> Json<Vec<Block>> {
    BLOCKCHAIN.lock().unwrap().add_block(q.data.clone());
    Json(BLOCKCHAIN.lock().unwrap().chain.clone())
}

pub async fn run() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/chain", get(get_chain))
        .route("/mine", post(add_block));
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Server at http://{addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
