use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping my box")
    }
}

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

    let mut x = 5;
    let z = MyBox::new(x);
    let w = MyBox::new(x);
    drop(w);

    println!("code");
    x = 200;

    let y = &x;

    assert_eq!(x, 200);
    assert_eq!(*y, 200);
    assert_eq!(*z, 5);
}
