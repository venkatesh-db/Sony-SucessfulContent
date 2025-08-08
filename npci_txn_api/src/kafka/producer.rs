
use rskafka::client::{Client, ClientConfig};
use std::env;

pub async fn send_event(topic: &str, msg: &str) {
    let brokers = env::var(\"KAFKA_BROKERS\").unwrap_or(\"localhost:9092\".into());
    let client = Client::new(ClientConfig::new(vec![brokers])).await.unwrap();
    let producer = client.producer(topic.to_string()).unwrap();
    producer.send(msg.into()).await.unwrap();
}
