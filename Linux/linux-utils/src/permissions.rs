
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn run() {
    let path = "demo.txt";
    match fs::metadata(path) {
        Ok(metadata) => {
            let permissions = metadata.permissions();
            println!("Permissions: {:o}", permissions.mode());
        }
        Err(e) => println!("Error: {}", e),
    }
}
