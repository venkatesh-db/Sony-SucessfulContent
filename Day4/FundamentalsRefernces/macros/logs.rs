
/*

chrono = "0.4"
colored = "2"
*/

use chrono::Local;
use colored::*;

/// Macro to log an action with timestamp and colored output
macro_rules! log_action {
    ($action:expr, $detail:expr) => {{
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let action = $action.to_uppercase();
        let formatted = format!("[{}] [{}]: {}", timestamp, action, $detail);

        // Apply color based on action type
        if action == "ERROR" {
            println!("{}", formatted.red());
        } else if action == "SUCCESS" {
            println!("{}", formatted.green());
        } else if action == "WARN" {
            println!("{}", formatted.yellow());
        } else {
            println!("{}", formatted.blue());
        }
    }};
}

fn main() {
    log_action!("info", "Application started");
    log_action!("success", "User registration completed");
    log_action!("warn", "Low disk space");
    log_action!("error", "Failed to connect to database");
    log_action!("debug", "Running diagnostics");
}
