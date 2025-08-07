
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

/// Wrapper around UnsafeCell to unsafely implement Sync
struct UnsafeWrapper(UnsafeCell<i32>);

// Manually implement Sync – saying: "I promise it's okay to share across threads"
unsafe impl Sync for UnsafeWrapper {}

fn main() {
    let data = Arc::new(UnsafeWrapper(UnsafeCell::new(0)));

    let data1 = Arc::clone(&data);
    
    let handle1 = thread::spawn(move || {
        for _ in 0..1000 {
            unsafe {
                *data1.0.get() += 1;
            }
        }
    });

    let data2 = Arc::clone(&data);
    let handle2 = thread::spawn(move || {
        for _ in 0..1000 {
            unsafe {
                *data2.0.get() += 1;
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let result = unsafe { *data.0.get() };
    println!("Final value: {}", result); // Likely not 2000 due to race
}
