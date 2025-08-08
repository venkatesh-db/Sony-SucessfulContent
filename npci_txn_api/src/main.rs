mod routes;
mod handlers;
mod db;
mod models;

use actix_web::{App, HttpServer, web};
use db::init;
use dotenv::dotenv;
use std::env;
use rdkafka::config::ClientConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db = init().await.expect("DB Failed");
    let kafka_producer = ClientConfig::new()
        .set("bootstrap.servers", &env::var("KAFKA_BROKERS").expect("Missing KAFKA_BROKERS env"))
        .create::<rdkafka::producer::FutureProducer>()
        .expect("Kafka Producer failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(kafka_producer.clone()))
            .configure(routes::init)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
