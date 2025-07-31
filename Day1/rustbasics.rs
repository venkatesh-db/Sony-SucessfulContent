

fn main(){

    // fixed memeory 
    let breaks:i32= 2;
    let energy:&str ="cofee";
     let mut sugarlevel:i8 = 90;
    
     // continous fixed 
    let enerygdrink :[&str;4]= ["milk","water","sugar","coffe"];
   
    // dynamic groable memory 
    let mut names =String::from("energy");
     names.push_str("laddu");
     
     
     let mut marriagwedding= vec!["sweet1","sweet2","rice","satrter1"];
                             // index 0      1         2       3 
                             
      marriagwedding.push("sweet1");

      println!("unlimted food{:?} {:?} ",marriagwedding ,names );
      
      for i in &marriagwedding{ // Derrfeing  index get value
           println!("loop i {}",i);
      }

      for i in &marriagwedding{ // Derrfeing  index get value
           println!("loop i {}",i);
      }
  
    if energy == "cofee"{
          let  onecup:i8 =2;
          sugarlevel = sugarlevel+onecup;
          println!("coffee addiction {}",sugarlevel);
    }
    
    else if   energy == "tea"{     
          let  oneteacup:i8 =3; 
          sugarlevel = sugarlevel+oneteacup
    
    }else if   energy == "water"{

       println!("i am healthy")
    }
    
}