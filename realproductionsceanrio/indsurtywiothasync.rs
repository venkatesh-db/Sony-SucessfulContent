
use std::sync::{mpsc::{channel, Sender, Receiver}, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct LogEntry {
    id: usize,
    message: String,
}

fn start_producer(tx_input: Sender<LogEntry>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for i in 0..10 {
            let log = LogEntry {
                id: i,
                message: format!("Log #{}", i),
            };
            println!("[Producer] Generated: {:?}", log);
            tx_input.send(log).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    })
}

fn start_processor(
    name: &str,
    rx_input: Arc<Mutex<Receiver<LogEntry>>>,
    tx_output: Arc<Mutex<Sender<String>>>,
) -> thread::JoinHandle<()> {
    let name = name.to_string();
    thread::spawn(move || {
        loop {
            let log = {
                let rx = rx_input.lock().unwrap();
                rx.recv().ok()
            };

            match log {
                Some(log) => {
                    let processed = format!("[{}] Processed Log {}: {}", name, log.id, log.message);
                    tx_output.lock().unwrap().send(processed).unwrap();
                }
                None => {
                    println!("[{}] Input channel closed. Exiting.", name);
                    break;
                }
            }
        }
    })
}

fn start_logger(rx_output: Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for received in rx_output {
            println!("[Logger] => {}", received);
        }
        println!("[Logger] Output channel closed. Logger exiting.");
    })
}

fn main() {
    let (tx_input, rx_input) = channel::<LogEntry>();
    let (tx_output, rx_output) = channel::<String>();

    let rx_input = Arc::new(Mutex::new(rx_input));
    let tx_output = Arc::new(Mutex::new(tx_output));

    let producer = start_producer(tx_input);
    let processor_a = start_processor("Processor A", Arc::clone(&rx_input), Arc::clone(&tx_output));
    let processor_b = start_processor("Processor B", Arc::clone(&rx_input), Arc::clone(&tx_output));
    let logger = start_logger(rx_output);

    producer.join().unwrap();
    processor_a.join().unwrap();
    processor_b.join().unwrap();

    // Drop the Arc to close logger's input
    drop(tx_output);

    logger.join().unwrap();
}
