use std::borrow::Borrow;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

enum Status {
    Value(i32),
    None,
}

fn get_func() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| 10)
}

fn main() {
    println!("Do twice result {}", do_twice(add_one, 2));
    println!("Do twice result {}", do_twice(|x: i32| { x * 10 }, 2));

    let v = vec![1, 2, 3, 4, 5];
    let b: Vec<Status> = (0..10).map(Status::Value).collect();

    let f = get_func();

    println!("Get func result {}", f(10));
    println!("Get func result {}", get_func()(10));
}
