
#[derive(Debug)]
enum Status {
    Success(String),
    Error(u32),
}

 #[derive(Debug)]
enum Payment{
    Sucess,
    Failure,
}



fn main() {
   let atmtransactioon= Payment::Sucess;
   println!("{:?}",atmtransactioon );



    let result = Status::Success("Transaction Complete".to_string());
    println!("{:?}",result );


    match result {
        Status::Success(msg) => println!("✅ {}", msg),
        Status::Error(code) => println!("❌ Error Code: {}", code),
    }
}
