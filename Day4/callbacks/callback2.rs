
fn apply_callback<F>(x: i32, cb: F) -> i32
where
    F: Fn(i32) -> i32,
{
    cb(x) + 1
}

fn callaback <V>(hrcallback:V) ->&'static str 
where 
V: Fn(&str) -> &str,
{
    hrcallback("venkat")
}


fn main() {
    
    let result = apply_callback(3, |n| n * 10);
    println!("Result: {}", result); // Result: 31

   let rets= callaback(|x| x);
   println!("rets: {}", rets); 

}
