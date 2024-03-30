pub mod restaurant;

use restaurant::customer::eat_at_restaurant;
use restaurant::restaurant::{self as restaurant_module, hosting, serving, Dish};

fn food() {
    println!("food")
}
