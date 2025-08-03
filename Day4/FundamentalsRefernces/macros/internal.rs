use std::io::{self, Write};

fn main() {
    let name = "Alice";
    io::stdout().write_fmt(format_args!("smile"));
    io::stdout().write_fmt(format_args!("Hello, {}\n", name)).unwrap();
}

