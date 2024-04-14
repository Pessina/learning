use add_one;
use rand;

fn main() {
    let y = rand::random::<u8>();
    println!("{}", add_one::add_one(3) + y);
}
