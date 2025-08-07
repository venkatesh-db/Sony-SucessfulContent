
use std::{
    collections::HashMap,
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
};

// Logging Macro
macro_rules! log_action {
    ($action:expr, $detail:expr) => {
        println!("[{}]: {}", $action, $detail);
    };
}

// Trait for File Operations
trait FileOperations {

    fn store(&self, filename: String, content: Vec<u8>);
    fn load(&self, filename: &str) -> Option<Vec<u8>>;
    fn list_files(&self) -> Vec<String>;

}

// File Server Struct
struct FileServer {
    storage: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl FileOperations for FileServer {

    fn store(&self, filename: String, content: Vec<u8>) {

        let mut files = self.storage.lock().unwrap();
        files.insert(filename.clone(), content);
        log_action!("UPLOAD", format!("Stored file: {}", filename));
    
    }

    fn load(&self, filename: &str) -> Option<Vec<u8>> {

        let files = self.storage.lock().unwrap();
        files.get(filename).cloned()
    
    }

    fn list_files(&self) -> Vec<String> {

        let files = self.storage.lock().unwrap();
        files.keys().cloned().collect()
    
    }
}

// CLI Handler Function
fn main() {

  let server=  Arc::new(FileServer{
         storage: Arc::new(Mutex::new(HashMap::new())),
    } );

    loop {

        println!("\n1. Upload File");
        println!("2. Download File");
        println!("3. List Files");
        println!("4. Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();


        match choice.trim() {
            "1" => {

                let server_clone = Arc::clone(&server);
               
                thread::spawn(move || {

                    print!("Filename to Upload: ");
                    io::stdout().flush().unwrap();

                    let mut filename = String::new();
                    io::stdin().read_line(&mut filename).unwrap();

                    let filename = filename.trim().to_string();

                    print!("File Content (Text): ");
                    io::stdout().flush().unwrap();

                    let mut content: String = String::new();
                    io::stdin().read_line(&mut content).unwrap();

                    let content_bytes = content.trim().as_bytes().to_vec();

                    server_clone.store(filename, content_bytes);
                }).join().unwrap();
            }

            "2" => {
                print!("Filename to Download: ");
                io::stdout().flush().unwrap();
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim();

                if let Some(content) = server.load(filename) {
                    println!("File Content: {}", String::from_utf8_lossy(&content));
                } else {
                    println!("File Not Found.");
                }
            }

            "3" => {
                let files = server.list_files();
                println!("Files Stored: ");
                for file in files {
                    println!("- {}", file);
                }
            }

            "4" => {
                println!("Exiting...");
                break;
            }

            _ => println!("Invalid Choice."),
        }
    }
}
