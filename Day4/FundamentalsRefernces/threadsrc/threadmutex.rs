
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {


    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
    
        let counter = Arc::clone(&counter);
    
        let handle = thread::spawn(move || {
            
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // Output: 5
}
