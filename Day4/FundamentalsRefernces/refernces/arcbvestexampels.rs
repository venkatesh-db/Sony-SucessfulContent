
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let storage: Arc<Mutex<HashMap<String, Vec<u8>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let threads: Vec<_> = (0..3).map(|i| {
        let s = Arc::clone(&storage);
        thread::spawn(move || {
            let mut map = s.lock().unwrap();
            let key = format!("file{}", i);
            map.insert(key, vec![i, i + 1, i + 2]);
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }

    let map = storage.lock().unwrap();
    for (k, v) in map.iter() {
        println!("{}: {:?}", k, v);
    }
}
