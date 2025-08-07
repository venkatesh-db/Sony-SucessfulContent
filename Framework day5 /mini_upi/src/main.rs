
mod models;
mod errors;
mod logger;
mod app;

use app::UpiApp;
use errors::ErrorLoggable;

fn main() {
    let mut app = UpiApp::new();

    app.add_user("Venkatesh", 2000.0);
    app.add_user("Ravi", 1000.0);

    match app.transfer("Venkatesh", "Ravi", 500.0) {
        Ok(_) => println!("✅ Transfer completed."),
        Err(e) => e.log(),
    }

    match app.balance("Venkatesh") {
        Ok(b) => println!("💰 Venkatesh balance: ₹{}", b),
        Err(e) => e.log(),
    }

    match app.transfer("Venkatesh", "Ravi", 9999.0) {
        Ok(_) => println!("✅ Transfer completed."),
        Err(e) => e.log(),
    }

    match app.transfer("Ghost", "Ravi", 500.0) {
        Ok(_) => println!("✅ Transfer completed."),
        Err(e) => e.log(),
    }
}
