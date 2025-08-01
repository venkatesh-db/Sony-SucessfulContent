
mod sony
{

pub struct GreatCoders
{ // private
    smiles:i8,
    name:&'static str
}


impl GreatCoders
{
   pub  fn new( sc:i8, n:&'static str )->GreatCoders{
 
        return GreatCoders
        {
            smiles:78 ,
            name:n

        }
     }

  pub   fn read(&self){
        println!("{}", self.smiles);
        //=23;
     }

   pub  fn setter(&mut self, sc:i8){
         self.smiles = sc;
     }
}
}


fn main(){
 
  //let mut indra = GreatCoders{ smiles:25};
 
  // stack object 

  let mut indra = sony::GreatCoders::new(100,"smiles");
  indra.read();
  indra.setter(75);
  indra.read();

  //  stack object move happens here 
  let mut hari=indra;
 //  indra.read();

}
