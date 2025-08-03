
// Execution formula1 - Geeneric objects
fn Excutions1(){

    // Geeneric object's 
    let mut obj:Box<i32> = Box::new(5);
    let mut obj1:Vec<i32>  = Vec::new();
    obj1.push(5);

}


struct  dusetti
{
   smiling:i8,
   happy:&'static str 
}

impl dusetti{

    fn new() ->Self{
        println!("cosntructore family ");

        Self{
            smiling:70,
            happy:"venkatesh",
        }
    }

    fn insidefamily(&mut self){
         self.smiling = 8;
         println!("smiling family {} ", self.smiling);
    }
}

// Execution formula1 -  objects creation or methgod call 
fn Exceutions2(){

    let mut venkat=  dusetti::new();
    venkat.insidefamily();

}


//  USe methods to perform on DSA 

fn Exceutions3(){

    let mut happy=vec![1,2,3];
    println!("{}",happy.len());
    println!("{}",happy.capacity());
    
    happy.extend([5,6,7]);
    println!("{:?}",happy);

    for i in happy.iter_mut(){
         *i = *i+1;
          println!("{}",i);
    }

    let rets=happy.iter().position(|&x| x>5 );
    println!("{:?}",rets);

}

use std::rc::Rc;


fn Exceutions4(){

  let mut  obj=   Rc::new(vec![1,2,3]);
  let  obj2=Rc::clone(&obj);
  let  obj3=Rc::clone(&obj);

 if let Some(v)= Rc::get_mut(&mut obj) {
     v.push(7);
      println!(" hello {:?}",v);
 }
 println!(" {:?}",obj3);

}

fn Exceutions42(){

   let mut obj = Rc::new(vec![1, 2, 3]);

    // No clones yet, get_mut will succeed
    if let Some(v) = Rc::get_mut(&mut obj) {
        v.push(7);
        println!("hello {:?}", v); // Works
    }

    let obj2 = Rc::clone(&obj);
    println!("{:?}", obj2); // Prints: [1, 2, 3, 7]


}

use std::cell::RefCell;


fn Exceutions43(){

   let obj=  RefCell::new(5);
     *obj.borrow_mut()+= 7;
   println!("{:?}",obj.borrow());

}

use std::thread;
use std::sync::mpsc;

fn Exceutions5(){

    
       let (tx,rx)=mpsc::channel();

  let handles=thread::spawn(
                             ||{
                                 println!("🧵 Hello from the spawned thread!");
                             }
  );

   tx.send("happy").unwrap();
   let recv=rx.recv().unwrap();
   println!("{:?}",recv);
   handles.join().unwrap();

}



fn main(){

// Exceutions4();
// Exceutions42();
// Exceutions43();

   Exceutions5();

}