
enum Status {
    Active(u32),
    Inactive,
}

fn main() {
    let user_status = Status::Active(80);

    match user_status {
        Status::Active(level) if level >= 50 => println!("High-level Active User."),
        Status::Active(level) => println!("Low-level Active User."),
        Status::Inactive => println!("User is inactive."),
    }
}
