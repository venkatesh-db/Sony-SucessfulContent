

fn main(){


    let playstation= vec![String::from("c"),String::from("cpp"),String::from("rust")];
    
   for i in &playstation {
      println!("{}",i)
   }

    let mut jamesbond = playstation.clone();

     for i in &jamesbond {
      println!("{}",i)
   }

   let rest=jamesbond.get(6);
   println!("{:?}",rest);

   jamesbond.insert(0,"cfans".into());

     for i in &playstation {
      println!("playstation {}",i);
   }

     for i in &jamesbond {
      println!("jamesbond {}",i);
   }

}