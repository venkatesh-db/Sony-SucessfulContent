
fn main(){

    let names =String::from("energy");
    let mut name1= names;
    println!("hep {}",name1);
   // println!("hep {}",names);
    name1.push_str("no rice");
    println!("avoid {}",name1);
}