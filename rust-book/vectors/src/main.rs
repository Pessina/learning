use std::fmt;

enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Int(i) => write!(f, "Int: {}", i),
            Cell::Float(fl) => write!(f, "Float: {}", fl),
            Cell::Text(text) => write!(f, "Text: {}", text),
        }
    }
}

fn main() {
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    vec.push(5);
    vec.push(6);

    let third = &vec[2];
    println!("The third element is {}", third);

    let fourth = vec.get(3);
    match fourth {
        Some(value) => println!("The fourth element is: {value}"),
        None => println!("Nothing to get"),
    }

    for i in &vec {
        println!("{i}")
    }

    for i in &mut vec {
        *i = *i + 50;
    }

    for i in &vec {
        println!("{i}")
    }

    let enum_vec = vec![
        Cell::Int(32),
        Cell::Float(32.3),
        Cell::Text(String::from("string")),
    ];

    for i in &enum_vec {
        match i {
            Cell::Int(int) => println!("Integer {int}"),
            _ => println!("Any"),
        }
    }
}
