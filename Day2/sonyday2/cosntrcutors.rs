

struct GreatCoders
{
    smiles:i8,
    name:&'static str
}

impl GreatCoders
{
     fn new( sc:i8, n:&'static str )->GreatCoders{
 
        return GreatCoders
        {
            smiles:78 ,
            name:n

        }
     }

     fn read(&self){
        println!("{}", self.smiles);
        //=23;
     }

     fn setter(&mut self, sc:i8){
         self.smiles = sc;
     }
}

fn main(){
 
  //let mut indra = GreatCoders{ smiles:25};
 
  // stack object 

  let mut indra =GreatCoders::new(100,"smiles");
  indra.read();
  indra.setter(75);
  indra.read();

  //  stack object move happens here 
  let mut hari=indra;
   indra.read();

}
