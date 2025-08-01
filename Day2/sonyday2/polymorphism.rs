

trait Lunchchoice{

    fn taste(&self);
    fn pricing(&self);
}

struct shivasagar
{
    name :&'static str
}
impl shivasagar
{
     fn menu(&mut self,ask:&'static str){

          self.name = ask;
     }
     
}

impl Lunchchoice for shivasagar{

      fn taste(&self){
  
            println!("{}", self.name);
      }

    fn pricing(&self){

    }
    
}


fn main(){


    let mut venkat=shivasagar{name:"veg thali"};
     venkat.menu("non veg thali");
     venkat.taste();

}