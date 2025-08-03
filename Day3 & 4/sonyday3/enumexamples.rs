
enum Errorhandling
{
    Exception(String),
    None,

}

enum Results{
    Ok(()),
    Err(String),
}

enum  Options{

     Some(String),
     None,
}

fn main()
{
   let obj:Errorhandling = Errorhandling::None;

   match obj{
    Errorhandling::Exception(val) => println!("{}",val),
    Errorhandling::None => println!("finally block"),     
   }

   let obj1:Options = Options::Some("box is moved".into());
    
     match obj1{
    Options::Some(val) => println!("{}",val),
    Options::None => println!("finally block"),
     
   }

     let obj2:Results = Results::Err("we love rust".into());
    
     match obj2{
    Results::Ok(()) => println!("no erros sucess"),
    Results::Err(val) => println!("rust error {}",val),
     
   }

   let obj:Result<(),&str>= Ok(());

   match obj{

    Ok(())=>  println!("treat by sony"),
    Err(val)=>  println!("oepration jamon"),
   
   }

}
