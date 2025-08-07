
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    println!("🔌 Connecting to PS4 server...");
    let mut stream = TcpStream::connect("192.168.1.100:9020")
        .expect("❌ Can't connect. Make sure PS4 or server is up.");

    println!("📤 Sending command: getinfo");
    stream.write_all(b"getinfo\n").unwrap();

    let mut buffer = [0; 512];
    let size = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..size]);

    println!("📥 Received: {}", response);
}

