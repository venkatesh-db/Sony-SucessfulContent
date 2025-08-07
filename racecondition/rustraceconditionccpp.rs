
use std::thread;
use std::ptr;
use std::time::Duration;

fn main() {
    let mut data = Box::new(0); // heap allocated integer
    let ptr = &mut *data as *mut i32; // raw pointer to the data

    let handle1 = thread::spawn(move || {
        for _ in 0..1000 {
            unsafe {
                *ptr += 1; // unsafely mutate shared data
            }
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..1000 {
            unsafe {
                *ptr += 1; // unsafely mutate shared data
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final value: {}", *ptr); // Likely NOT 2000 due to race condition
    }
}
