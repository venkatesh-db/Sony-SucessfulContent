
#[dependencies]
#chrono = "0.4"

use chrono::Utc;

fn main() {
    let now = Utc::now();
    println!("Current UTC Time: {}", now);
}
