
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {

    let counter = Arc::new(AtomicUsize::new(0));
    
    let mut handles = vec![];

    for _ in 0..2 {

        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {

            for _ in 0..5 {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", counter.load(Ordering::Relaxed));
}
