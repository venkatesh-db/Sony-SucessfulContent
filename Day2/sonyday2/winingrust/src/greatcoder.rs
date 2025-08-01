


pub struct GreatCoders
{
    smiles:i8,
    name:&'static str
}

impl GreatCoders
{
    pub fn new( sc:i8, n:&'static str )->GreatCoders{
 
        return GreatCoders
        {
            smiles:78 ,
            name:n

        }
     }

   pub  fn read(&self){
        println!("{}", self.smiles);
        //=23;
     }

 pub    fn setter(&mut self, sc:i8){
         self.smiles = sc;
     }
}
