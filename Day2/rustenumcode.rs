
enum ShapeError{
    Invalidimension,
}

enum Payment<'a>{
    Sucess(&'a str),
    Failure(&'static str),
}

trait logger {
    fn log(&self, obj1:Payment);
}


impl logger for ShapeError{
        fn log(&self, obj1:Payment)
        {
            match obj1{

                 Payment::Sucess(val)=> println!("match Payment::Sucess {}",val ),
                 Payment::Failure(val)=>println!("match Failure {}",val ),
            }

            println!("smiles");
        }
}


fn main(){
   let obj= ShapeError::Invalidimension;
   let obj1= Payment::Sucess("venkatesh");
   logger::log(&obj,obj1);
}