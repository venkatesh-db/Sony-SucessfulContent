
use upi_benchmark::create_upi_payload;

fn main() {
    let payload = create_upi_payload("venkat@upi", "merchant@upi", 500.0);
    println!("✅ UPI Payload from main: {:?}", payload);
}
