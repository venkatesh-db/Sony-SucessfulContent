
trait Taste {
    fn taste(&self);
}

struct Idly;
impl Taste for Idly {
    fn taste(&self) {
        println!("Soft and fluffy!");
    }
}

struct Hotel {
    menu_item: Box<dyn Taste>,  // Trait object field
}

impl Hotel {
    fn describe(&self) {
        self.menu_item.taste();  // Dynamic dispatch
    }
}

fn main() {
    let item = Box::new(Idly);
    let hotel = Hotel { menu_item: item };
    hotel.describe();
}
