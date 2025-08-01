

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


struct RustCoders 
{
     obj:Box<GreatCoders> // objs is base class object
}

impl RustCoders
{
     
}

fn main(){
 
  //let mut indra = GreatCoders{ smiles:25};
 
  // stack object 

  let mut indra =Box::new(GreatCoders::new(100,"smiles"));

                 // line1 GreatCoders::new // line2 Box::new stack data is moved to heap


  indra.read();
  indra.setter(75);
  indra.read();

  //  stack object move happens here 
  let mut hari=indra;
   //indra.read();

   // superman is derived class object 
    let superman= Box::new(RustCoders{obj:Box::new(GreatCoders{ smiles:75,name:"fridays smiels"})});
                  // line 1 GreatCoders  line 2 Box::new(
    let fridayjoy=RustCoders{obj: hari };

    superman.obj.read();

}

