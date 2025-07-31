
struct  dusetti
{
   smiling:i8,
   happy:&'static str 
}

impl dusetti{

    fn new(){
        println!("cosntructore family ");
    }

    fn insidefamily(&mut self){
         self.smiling = 8;
         println!("smiling family {} ", self.smiling);
    }
}


fn main(){
    // var venkat dusetti
    // dusetti venkatesh ( )

    let mut venkatesh = dusetti{smiling:5,happy:"smiling"};
    // stack --> i8 , &'static str --> data segements
   let manjunath= Box::new( dusetti{smiling:5,happy:"smiling"}) ;

    println!(" {} ", manjunath.smiling);
    println!(" {} ", venkatesh.smiling);
    println!(" {} ", venkatesh.happy);
    println!(" {:p} ", &venkatesh.happy);
    println!(" {:p} ", &manjunath.happy);

    venkatesh.insidefamily();

    dusetti::new();
}