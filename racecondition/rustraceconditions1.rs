
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let data1 = Arc::clone(&data);

    let handle1 = thread::spawn(move || {
          println!("thread1");
        for _ in 0..5 {
            let mut val = data1.lock().unwrap();
            *val += 1;
        }
    });


    let handle2 = thread::spawn(move || {
           let data2 = Arc::clone(&data);
        for _ in 0..2 {
            println!("thread2");
            let mut val = data2.lock().unwrap();
            *val += 1;
        }
    });
    println!("main is joining now two threads");

    handle1.join().unwrap();

   // handle2.join().unwrap();

   println!("main iam waiting for jamon");

    println!("Final value: {}", *data.lock().unwrap()); // Will always be 2000
}
