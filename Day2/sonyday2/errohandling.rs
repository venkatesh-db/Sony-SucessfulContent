
enum Focused
{
FOCUS, // 0
BREAK, // 1
DOUBT, // 2 
}


enum FocusedProd
{
workless(i32), //  level1
enjoy(String),
}


// workless(i32) // level1 - type of variable 
// let obj2: FocusedProd =workless (7)  --> insitalied the variable 
// match obj2 
// workless(val) 


fn main(){


let  obj:Focused =Focused::FOCUS;

match obj{
       Focused::FOCUS => println!(" focused now"),
       Focused::BREAK => println!(" break neeeded now"),
       Focused::DOUBT=> println!(" helped needed now"),
} 


let smiles:i32= 36777;

let  obj1:FocusedProd = FocusedProd::enjoy("spend employees".to_string());

let obj2: FocusedProd = FocusedProd::workless(7);  // level2


match obj1{

FocusedProd::workless(val)=>println!("finishwork{}",val),

FocusedProd::enjoy(val1)=>println!("enjoy & distrubing others{}",val1),

}


}
