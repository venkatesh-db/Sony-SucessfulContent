
fn fn3( n2:&Box<i8> ){

 // copy inside struct object 
 let transfromations1 = Tranformation{n:"npci@123",names:"npci-transofromation".to_string(),n2:Box::new(**n2)};
 
  println!(" {:?} ",transfromations1);

  
}

#[derive(Debug)]
struct Tranformation{
      n:&'static str,
      names:String ,
      n2:Box<i8>,
 } 


fn fn2(smil:&Tranformation){

 // fn2 to fn3 
 
fn3( &smil.n2)

}

 fn fn1(){

 
 let smiles = Tranformation{n:"venakt",names:"npci".to_string(),n2:Box::new(7)};

   println!(" before trasnformations {:?} ",smiles);

  fn2(&smiles)


}

fn main(){

    fn1();

}