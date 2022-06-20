// crate được khai báo trong file .toml (crates.io)
use rand::{RngCore, Rng, SeedableRng};

mod back_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    pub enum Salad {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                fruit: String::from("Banana") 
            }
        }
    }
}

// Khai báo module, tuy nhiên nội dung của module này nằm ở file front_house.rs
mod front_house;

fn eat_res() {
    front_house::hosting::add_to_waitlist();
}

fn eat_at_restaurant () {
    let mut order = back_house::Breakfast::monday("Fish");
    order.toast = String::from("Chicken");

    let order2 = back_house::Breakfast {
        toast: String::from("Wheat"),
        fruit: String::from("Apple"),
    };

    let order2 = back_house::Salad::Salad;
}


