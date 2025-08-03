
mod NameCollison
{
pub fn smile()
{
  println!("mod smile");
}
}

fn smile(){

  println!(" smile");
}

trait Database
{ // resublity , implenentation 
     fn writeintommysql(&self)
     {
          println!(" write in to db");
     }

     fn schema(&self);

}

//#[derive(Debug)]
struct sonydb
{
   fun:&'static str
}

impl Drop for sonydb{

     fn drop(&mut self){
          println!("i am your super herofor resourcess ");
     }
}

#[derive(Debug)]
struct Embedded
{
  name: String,
}

impl Clone for Embedded{

     fn clone(&self)->Self {
          println!("i am doing deeep copy for heap memory");
          return Embedded{name:self.name.clone()}
     }
}

impl std::fmt::Display for sonydb{

     fn fmt(&self,f:&mut std::fmt::Formatter ) ->std::fmt::Result {
              println!("super man is here");
              write!( f,"smiling rust coders {}",self.fun)
     }
}

impl Database for sonydb{

/* 
     fn writeintommysql(&self)
     {
          println!(" sony in to db");
     }
*/
     fn schema(&self)
     {
             println!(" sony scema");
     }
}


impl 
{
     
}

fn main(){

    smile();
    NameCollison::smile();

  //  Database::writeintommysql();
 {
      let saturdaysmiles = sonydb{fun:"saturday jolly day"};

        let u1 = Embedded { name: "karthik".into() };
        let u2 = u1.clone();
          println!("u2 cloned {:?}",u2);
 }
    let gowtham = sonydb{fun:"saturday jolly day"};
    gowtham.writeintommysql();
    gowtham.schema();

    println!("{}",gowtham);

}