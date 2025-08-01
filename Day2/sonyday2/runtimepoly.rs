
trait Lunchchoice {
    fn taste(&self);
    fn pricing(&self);
}

struct Sony {
    discount: f32,
}

impl Lunchchoice for Sony {
    fn taste(&self) {
        println!("Sony taste: fusion meals");
    }

    fn pricing(&self) {
        println!("Sony pricing: ₹{}", 100.0 - self.discount);
    }
}

struct Laksmisagar {
    food: &'static str,
}

impl Lunchchoice for Laksmisagar {
    fn taste(&self) {
        println!("Laksmisagar taste: {:?}", self.food);
    }

    fn pricing(&self) {
        println!("Laksmisagar pricing: ₹50");
    }
}

// ✅ FIX: Remove generic type from trait object
fn dynamics(bond: &dyn Lunchchoice) {
    bond.taste();
    bond.pricing();
}

fn main() {
    let venkatesh = Box::new(Laksmisagar { food: "2 chapati" });

    // ✅ FIX: deref Box to get trait object reference
    dynamics(&*venkatesh);
}

trait Lunchchoice {
    fn taste(&self);
    fn pricing(&self);
}

struct Sony {
    discount: f32,
}

impl Lunchchoice for Sony {
    fn taste(&self) {
        println!("Sony taste: fusion meals");
    }

    fn pricing(&self) {
        println!("Sony pricing: ₹{}", 100.0 - self.discount);
    }
}

struct Laksmisagar {
    food: &'static str,
}

impl Lunchchoice for Laksmisagar {
    fn taste(&self) {
        println!("Laksmisagar taste: {:?}", self.food);
    }

    fn pricing(&self) {
        println!("Laksmisagar pricing: ₹50");
    }
}

// ✅ Accepts Box<dyn Lunchchoice>, no need to manually deref
fn dynamics(bond: Box<dyn Lunchchoice>) {
    bond.taste();
    bond.pricing();
}

fn main() {
    let venkatesh = Box::new(Laksmisagar { food: "2 chapati" });
    dynamics(venkatesh); // ✅ Works fine
}

 

*/