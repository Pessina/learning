pub mod hosting {
    pub fn print_hi() {
        println!("Hi");
        super::super::super::food();
    }
}

pub mod serving {
    fn print_bye() {
        println!("Bye")
    }
}

pub struct Dish {
    pub portion: u32,
    is_beef: bool,
}

impl Dish {
    pub fn new_dish(portion: u32) -> Self {
        Dish {
            portion,
            is_beef: false,
        }
    }
}
