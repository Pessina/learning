use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(10, Rc::new(List::Cons(8, Rc::new(List::Nil)))));
    println!("Count on a = {}", Rc::strong_count(&a));

    let b = List::Cons(10, Rc::clone(&a));
    println!("Count on b = {}", Rc::strong_count(&a));

    {
        let c = List::Cons(10, Rc::clone(&a));
        println!("Count on c = {}", Rc::strong_count(&a));
    }

    println!("Count drop c = {}", Rc::strong_count(&a));
}
