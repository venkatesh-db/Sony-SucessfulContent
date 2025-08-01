

pub trait Taste {
    fn taste(&self);
}

pub struct Idly;

impl Taste for Idly {
    fn taste(&self) {
        println!("Soft and fluffy!");
    }
}

pub struct Hotel {
  pub  menu_item: Box<dyn Taste>,  // Trait object field
}

impl Hotel {
  pub  fn describe(&self) {
        self.menu_item.taste();  // Dynamic dispatch
    }
}