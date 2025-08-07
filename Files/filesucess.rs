
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("lines.txt")?;
    let mut writer = BufWriter::new(file);

    for i in 1..=5 {
        writeln!(writer, "Line number {}", i)?;
    }

    Ok(())
}
