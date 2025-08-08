
use redis::Client;

pub fn connect(uri: &str) -> Client {
    Client::open(uri).expect("Failed to connect to Redis")
}