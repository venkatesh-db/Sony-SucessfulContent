
mod NPCI{

pub fn Function( n:&str) ->&str{
    println!("fucntion {}",n);
    return n;
} 

}

fn main(){

let ret:&str;
ret=NPCI::Function("smiles");
let boxs:Box<i32> = Box::new(5);

}