
mod greatcoder;
mod weekendgoal;


// use  crate::weekendgoal::{ Taste , Idly ,Hotel };

 use crate::weekendgoal::poly::Taste;
  use crate::weekendgoal::poly::Idly;
   use  crate::weekendgoal::poly::Hotel;

use crate::greatcoder::GreatCoders;

fn main(){
 
  //let mut indra = GreatCoders{ smiles:25};
 
  // stack object 

  let mut indra =GreatCoders::new(100,"smiles");
  indra.read();
  indra.setter(75);
  indra.read();

  //  stack object move happens here 
  let mut hari=indra;
   //indra.read();

   let item = Box::new(Idly);
    let hotel = Hotel { menu_item: item };
    hotel.describe();
   
}
