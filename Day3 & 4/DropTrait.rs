
struct Connection;

impl Drop for Connection {
    fn drop(&mut self) {
        println!("Connection closed automatically.");
    }
}

fn main() {
    let _conn = Connection;
    println!("Using connection...");
}
