
use crate::services::{db, redis, kafka};
use crate::config::Config;
use std::sync::Arc;
use scylla::Session;
use redis::Client as RedisClient;
use rdkafka::producer::FutureProducer;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Session>,
    pub redis: Arc<RedisClient>,
    pub kafka: Arc<FutureProducer>,
}

impl AppState {
    pub async fn init(config: &Config) -> Self {
        let db = db::connect(&config.cassandra_uri).await;
        let redis = redis::connect(&config.redis_uri);
        let kafka = kafka::connect(&config.kafka_broker);

        AppState {
            db: Arc::new(db),
            redis: Arc::new(redis),
            kafka: Arc::new(kafka),
        }
    }
}