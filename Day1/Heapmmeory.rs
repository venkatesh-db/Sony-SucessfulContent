

fn valuebowowring( mut copysmiles: i32 , boworrer:*mut i32 ){
     
     println!("ownership {}",copysmiles);
     println!("ownership address {:p} {:p}",&boworrer ,boworrer);
     copysmiles =7;
     unsafe{
     *boworrer =77;
      println!("ownership {}",*boworrer);
     }
}

fn ownership( mut smart: Box<i32>){


     
    *smart = 107;
    
     let p:  *mut i32 = &mut *smart;
      println!(" ownership deref {}",smart);


      unsafe{
         println!(" ownership deref {} {:p} {:p}  ", *p, p , &p);
      }
  
    
}// bond1 call destructor , drop trait

fn heapmemory(){

 let mut bond  =  Box::new(7);
 //stack       // function call  in heap  

 *bond =9;
 
 println!(" first deref {}",bond);

 let mut bond1 =bond;
 //println!("heapmemory {}",bond);
 
 *bond1 =99;
 
  println!(" second deref {}",bond1);
 //bond= Box::new(9);
 
 ownership( bond1  ) // In this scope no owner to heap memory
 
} 

fn newstring( name:&str ){
    
     println!("newstring {:p} {}",&name , name);
}

fn strings(){

   let name:&str = "smiling sony";
   //  stack            // Ro data , code/text 
   //name[0]='v';
      println!("strings {:p} {}",&name , name);

    newstring( name)

    println!("end smile strings {:p} {}",&name , name);
}



fn main(){

/*
  let mut smiles: i32 =5;
  smiles=6;
  println!("{}",smiles);
  valuebowowring( smiles , &mut smiles);
   println!("end of main {}",smiles);
      println!("end of main {:p}",&smiles);
*/
   // heapmemory();
}

