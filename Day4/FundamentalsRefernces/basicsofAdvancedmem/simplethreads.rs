
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("🧵 Hello from the spawned thread!");
    });

    println!("🧑 Hello from the main thread!");

    handle.join().unwrap(); // Wait for the thread to finish
}
