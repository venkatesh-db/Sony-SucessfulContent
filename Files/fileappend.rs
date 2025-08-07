
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("hello.txt")?;

    file.write_all(b"\nAppended line")?;
    Ok(())
}
