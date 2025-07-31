
use std::f64::consts::PI;

// Custom Error
#[derive(Debug)]
enum ShapeError {
    InvalidDimension,
}

// Shape Enum with area() method
enum Shape {
    Circle(f64),     // radius
    Rectangle(f64, f64),  // width, height
}

impl Shape {
    fn area(&self) -> Result<f64, ShapeError> {
        match *self {
            Shape::Circle(r) if r > 0.0 => Ok(PI * r * r),
            Shape::Rectangle(w, h) if w > 0.0 && h > 0.0 => Ok(w * h),
            _ => Err(ShapeError::InvalidDimension),
        }
    }
}

// Custom Logger Trait
trait Logger {
    fn log(&self);
}

impl Logger for ShapeError {
    fn log(&self) {
        eprintln!("❌ Error: {:?}", self);
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(10.0, 4.0),
        Shape::Circle(-1.0), // Invalid
    ];

    for shape in shapes {
        match shape.area() {
            Ok(a) => println!("Area: {:.2}", a),
            Err(e) => e.log(),  // Using Custom Logger Trait
        }
    }
}
