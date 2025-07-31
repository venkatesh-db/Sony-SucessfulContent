
fn library(  family:(&str,&str ,&str)){
 println!(" Libarray {} {} {}" ,family.0,family.1,family.2 );
}

fn arrayboworring( arr:&[i32]){

println!("array borrowing: {:?}", arr);

}


fn main() {
    // ✅ Tuple Data Type
    let person: (u32, &str, bool) = (101, "Venkatesh", true);
    println!("ID: {}, Name: {}, Active: {}", person.0, person.1, person.2);

    let mut family:(&str,&str ,&str) = ("venkat","manjunath","babu");
    println!("{} {} {}" ,family.0,family.1,family.2 );

    family.0 = "venkateswara";
    library(("venkat","manjunath","babu"));

     println!("{} {} {}" ,family.0,family.1,family.2 );

    // ✅ Array Data Type
    let scores: [i32; 4] = [90, 85, 77, 92];
    println!("First Score: {}", scores[0]);

    arrayboworring(&scores[2..3]);

    // ✅ Slice Data Type
    let slice_of_scores: &[i32] = &scores[1..3];  // slice from index 1 to 2
    println!("Slice: {:?}", slice_of_scores);

    // Iterate over slice
    for score in slice_of_scores {
        println!("Sliced Score: {}", score);
    }
}
