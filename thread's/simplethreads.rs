
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("From thread: {}", i);
        }
    });

    for i in 1..=5 {
        println!("From main: {}", i);
    }

    handle.join().unwrap();
}
