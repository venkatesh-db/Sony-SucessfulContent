
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Sample log structure
#[derive(Debug, Clone)]
struct LogEntry {
    id: usize,
    message: String,
}

fn main() {
    // Channel from producer -> processors
    let (tx_input, rx_input) = channel::<LogEntry>();

    // Shared output channel to logger
    let (tx_output, rx_output) = channel::<String>();

    // Wrap sender in Arc for both processors
    let tx_output = Arc::new(Mutex::new(tx_output));

    // Thread 1: Producer (simulates incoming logs)
    let producer = thread::spawn(move || {
        for i in 0..10 {
            let log = LogEntry {
                id: i,
                message: format!("Log #{}", i),
            };
            println!("[Producer] Generated: {:?}", log);
            tx_input.send(log).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Clone input receiver for both processors
    let rx_input = Arc::new(Mutex::new(rx_input));

    // Thread 2: Processor A
    let rx1 = Arc::clone(&rx_input);
    let tx1 = Arc::clone(&tx_output);
    let processor_a = thread::spawn(move || {
        loop {
            let log = {
                let rx = rx1.lock().unwrap();
                rx.recv().ok()
            };

            match log {
                Some(log) => {
                    let processed = format!("[A] Processed Log {}: {}", log.id, log.message);
                    tx1.lock().unwrap().send(processed).unwrap();
                }
                None => break,
            }
        }
    });

    // Thread 3: Processor B
    let rx2 = Arc::clone(&rx_input);
    let tx2 = Arc::clone(&tx_output);
    let processor_b = thread::spawn(move || {
        loop {
            let log = {
                let rx = rx2.lock().unwrap();
                rx.recv().ok()
            };

            match log {
                Some(log) => {
                    let processed = format!("[B] Processed Log {}: {}", log.id, log.message);
                    tx2.lock().unwrap().send(processed).unwrap();
                }
                None => break,
            }
        }
    });

    // Thread 4: Logger / Output
    let logger = thread::spawn(move || {
        for received in rx_output {
            println!("[Logger] => {}", received);
        }
    });

    producer.join().unwrap();
    processor_a.join().unwrap();
    processor_b.join().unwrap();

    // Drop output channel to exit logger loop
    drop(tx_output);

    logger.join().unwrap();
}
