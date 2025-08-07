
use std::fs;

pub fn run() {
    let path = ".";
    println!("Files in {}:", path);
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                println!("-> {}", entry.path().display());
            }
        }
        Err(e) => println!("Error reading dir: {}", e),
    }
}
