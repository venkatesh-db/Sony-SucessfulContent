mod list_dir;
mod file_io;
mod process;
mod network;
mod permissions;
mod mount;
mod memory_map;

use std::io::{self, Write};

fn main() {
    println!("Linux Utils CLI - Select an option:");
    println!("1. List Directory");
    println!("2. File I/O");
    println!("3. Spawn Process");
    println!("4. TCP Client");
    println!("5. File Permissions");
    println!("6. Mount Info");
    println!("7. Memory Map File");
    println!("0. Exit");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => list_dir::run(),
        "2" => file_io::run(),
        "3" => process::run(),
        "4" => network::run(),
        "5" => permissions::run(),
        "6" => mount::run(),
        "7" => memory_map::run(),
        "0" => println!("Bye!"),
        _ => println!("Invalid choice"),
    }
}
