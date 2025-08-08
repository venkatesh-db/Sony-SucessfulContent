
use std::env;

#[derive(Clone)]
pub struct Config {
    pub cassandra_uri: String,
    pub redis_uri: String,
    pub kafka_broker: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            cassandra_uri: env::var("CASSANDRA_URI").unwrap(),
            redis_uri: env::var("REDIS_URI").unwrap(),
            kafka_broker: env::var("KAFKA_BROKER").unwrap(),
        }
    }
}