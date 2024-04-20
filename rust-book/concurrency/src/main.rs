use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("print v {:?}", v);
        for i in 1..10 {
            println!("Number printed inside the thread {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number printed on main {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
