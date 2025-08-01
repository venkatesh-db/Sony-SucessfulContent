
struct GreatCoders
{
    smiles:i8
}

impl GreatCoders
{
     fn read(&self){
        println!("{}", self.smiles);
        //=23;
     }

     fn setter(&mut self, sc:i8){
         self.smiles = sc;
     }
}

fn main(){
 
  let mut indra = GreatCoders{ smiles:25};
  indra.read();
  indra.setter(75);
  indra.read();

}
