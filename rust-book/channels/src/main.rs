use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vec = [
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
        ];

        for i in vec {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vec = [
            String::from("Hi"),
            String::from("my"),
            String::from("name"),
            String::from("is"),
            String::from("Felipe"),
        ];

        for i in vec {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("Got: {}", recv);
    }
}
