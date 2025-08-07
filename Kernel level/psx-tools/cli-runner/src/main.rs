use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .expect("failed to execute process");

    println!("Status: {}", output.status);
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
}
