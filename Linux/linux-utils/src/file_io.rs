
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn run() {
    let filename = "demo.txt";
    let mut file = OpenOptions::new().create(true).write(true).open(filename).unwrap();
    writeln!(file, "Hello from Rust Linux Utils!").unwrap();

    let mut contents = String::new();
    File::open(filename).unwrap().read_to_string(&mut contents).unwrap();

    println!("File Contents:\n{}", contents);
}
