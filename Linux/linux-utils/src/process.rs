
use std::process::Command;

pub fn run() {
    println!("Running `ls -la`...");
    let output = Command::new("ls").arg("-la").output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
