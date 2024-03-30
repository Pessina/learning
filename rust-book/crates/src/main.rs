use garden::vegetables::Apple;

pub mod garden;

fn main() {
    let apple = Apple { size: 32 };
    println!("My apple: {:#?}", apple);
}
