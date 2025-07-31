
use std::ops::Add;

struct Point(i32, i32);

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

fn main() {
    let p1 = Point(2, 3);
    let p2 = Point(4, 5);
    let result = p1 + p2;

    println!("Point: ({}, {})", result.0, result.1);
}
