use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3).map(|i| {
        let shared = Arc::clone(&data);
        thread::spawn(move || {
            let mut vec = shared.lock().unwrap();
            vec.push(i);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("{:?}", data.lock().unwrap());
}
