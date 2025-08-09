
// once_cell = "1.18"

use once_cell::sync::Lazy;
use std::sync::Mutex;

struct AppConfig {
    name: String,
}

static CONFIG: Lazy<Mutex<AppConfig>> = Lazy::new(|| {
    Mutex::new(AppConfig {
        name: "MyApp".into(),
    })
});

fn main() {

    {
    let mut config = CONFIG.lock().unwrap();
    println!("App name: {}", config.name);
    config.name = "NewName".to_string();
    println!("App name1: {}", config.name);
    }
    {
    let mut config2 = CONFIG.lock().unwrap();
     config2.name = "smiling npci".to_string();
     println!("App name2: {}", config2.name);
    }
}
