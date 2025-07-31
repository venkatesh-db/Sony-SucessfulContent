
# uuid = { version = "1.8", features = ["v4"] }

use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v4();
    println!("Generated UUID v4: {}", my_uuid);
}
