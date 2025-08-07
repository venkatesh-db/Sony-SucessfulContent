
use std::fs;

pub fn run() {
    println!("Mount points from /proc/mounts:");
    match fs::read_to_string("/proc/mounts") {
        Ok(contents) => {
            for line in contents.lines().take(10) {
                println!("{}", line);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
