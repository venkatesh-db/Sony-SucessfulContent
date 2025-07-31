
fn main() {
    let marks = 75;

    // ✅ if
    if marks >= 90 {
        println!("Grade: A");
    } else if marks >= 60 {
        println!("Grade: B");
    } else {
        println!("Grade: C");
    }

    // ✅ match with Pattern Guard
    match marks {
        m if m >= 90 => println!("Excellent!"),  // Pattern Guard (m if m >= 90)
        m if m >= 60 => println!("Good!"),
        _ => println!("Needs Improvement."),
    }
}
