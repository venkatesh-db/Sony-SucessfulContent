
use std::sync::mpsc;
use std::thread;


fn main() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        rx.send("Message from sony experts").unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());
}
