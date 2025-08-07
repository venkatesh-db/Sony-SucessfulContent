
use std::net::TcpStream;
use std::io::{Read, Write};

pub fn run() {
    println!("Connecting to example.com:80...");
    if let Ok(mut stream) = TcpStream::connect("example.com:80") {
        let _ = stream.write_all(b"GET / HTTP/1.0\r\n\r\n");
        let mut buf = [0; 512];
        let size = stream.read(&mut buf).unwrap();
        println!("Response: {}", String::from_utf8_lossy(&buf[..size]));
    } else {
        println!("Connection failed.");
    }
}
