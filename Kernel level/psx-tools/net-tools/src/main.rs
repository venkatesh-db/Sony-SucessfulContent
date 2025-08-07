
#[cfg(target_os = "linux")]
fn run_linux_cmd() {
    let output = std::process::Command::new("ip")
        .args(&["addr", "show"])
        .output()
        .expect("Failed");

    println!("Linux Output: {}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(target_os = "macos")]
fn run_macos_cmd() {
    let output = std::process::Command::new("ifconfig")
        .output()
        .expect("Failed");

    println!("macOS Output: {}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    #[cfg(target_os = "linux")]
    run_linux_cmd();

    #[cfg(target_os = "macos")]
    run_macos_cmd();
}
