mod config;
mod app_state;
mod handlers;
mod services;
mod models;

use actix_web::{App, HttpServer, web};
use crate::handlers::user_handler::get_user;
use app_state::AppState;
use config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::from_env();
    let state = AppState::init(&config).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/user/{id}", web::get().to(get_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}