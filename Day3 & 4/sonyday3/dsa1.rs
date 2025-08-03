
fn playstation(  smartpointer:&mut Vec<&str>){

   smartpointer.push(" heavy dinner");

}

fn teamcamera(   mut smartpointer:Vec<&str>){

     smartpointer.push(" heavy lunch");

      smartpointer = vec!["c","cpp"];

    playstation(&mut smartpointer);

    for i in &smartpointer{
    println!("{}",i);
   }
}


fn main(){


   let mut smartpointer=  Vec::new();
   let smp =   vec![1,2,3];


   smartpointer.push("smiling lunch");
   for i in &smartpointer{
    println!("{}",i);
   }

   for i in &smartpointer{
    println!("{}",i);
   }
 
  teamcamera(smartpointer.clone());

    for i in &smartpointer{
    println!("post bowrring {}",i);
   }

}