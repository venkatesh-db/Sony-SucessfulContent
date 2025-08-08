
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;

pub fn connect(broker: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", broker)
        .create()
        .expect("Kafka producer creation failed")
}