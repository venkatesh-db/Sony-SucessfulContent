
trait LunchChoice {
    fn taste(&self);
}

trait PremiumLunch: LunchChoice {
    fn pricing(&self);
}

// ✅ Implement base trait for struct
struct Sony;
impl LunchChoice for Sony {
    fn taste(&self) {
        println!("Sony lunch tastes great!");
    }
}

// ✅ Implement subtrait for struct
impl PremiumLunch for Sony {
    fn pricing(&self) {
        println!("Sony pricing: ₹100");
    }
}

// ✅ Function using the subtrait (inherits taste() too)
fn show_premium(lunch: &dyn PremiumLunch) {
    lunch.taste();   // from LunchChoice
    lunch.pricing(); // from PremiumLunch
}

fn main() {
    let sony = Sony;
    show_premium(&sony);
}
