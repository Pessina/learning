use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let b = List::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    *value.borrow_mut() = 300;

    println!("Value for a: {:?}", a);
    println!("Value for a: {:?}", b);
    println!("Value for a: {:?}", c);
}
