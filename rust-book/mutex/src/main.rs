use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("counter = {:?}", *counter.lock().unwrap());
}
