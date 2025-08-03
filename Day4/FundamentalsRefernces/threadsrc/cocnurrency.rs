
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn thread 1
    thread::spawn(|| {
        for i in 1..=5 {
            println!("🔁 Thread 1: Count {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Spawn thread 2
    thread::spawn(|| {
        for i in 1..=5 {
            println!("🔁 Thread 2: Count {}", i);
            thread::sleep(Duration::from_millis(700));
        }
    });

    // Main thread continues concurrently
    for i in 1..=5 {
        println!("👑 Main thread: Count {}", i);
        thread::sleep(Duration::from_millis(400));
    }

    // Sleep main long enough to let other threads finish
    thread::sleep(Duration::from_secs(3));
}
