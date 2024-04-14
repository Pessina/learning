enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    let x = 5;
    println!("{:p}, {}", b, x);

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}
