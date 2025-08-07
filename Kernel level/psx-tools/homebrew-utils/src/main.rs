
use std::fs;

fn list_project_files() {
    let path = r"/Users/venkatesh/Sony /Day6/Kernel level/psx-tools/homebrew-utils";
    match fs::read_dir(path) {
        Ok(entries) => {
            println!("Files in {}:", path);
            for entry in entries {
                if let Ok(file) = entry {
                    println!("-> {}", file.path().display());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read folder '{}': {}", path, e);
        }
    }
}

fn main() {
    list_project_files();
}
