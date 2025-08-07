
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};

fn main() -> std::io::Result<()> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("unique.txt")
    {
        Ok(mut file) => {
            file.write_all(b"Created only if not exists")?;
        }
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                println!("File already exists!");
            } else {
                return Err(e);
            }
        }
    }

    Ok(())
}
