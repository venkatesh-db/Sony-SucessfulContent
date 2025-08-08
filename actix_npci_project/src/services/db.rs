
use scylla::{Session, SessionBuilder};

pub async fn connect(uri: &str) -> Session {
    SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .expect("Failed to connect to ScyllaDB")
}