use crate::restaurant::restaurant;

pub fn eat_at_restaurant() {
    restaurant::hosting::print_hi();

    restaurant::hosting::print_hi();

    let dish = restaurant::Dish::new_dish(3);
}
