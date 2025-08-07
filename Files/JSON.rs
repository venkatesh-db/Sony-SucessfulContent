
use std::fs::File;
use serde_json::json;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let data = json!({
        "name": "Venkatesh",
        "role": "Rustacean"
    });

    let mut file = File::create("data.json")?;
    write!(file, "{}", data.to_string())?;
    Ok(())
}
