use core::fmt;
use std::{fmt::write, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub trait Iterator {
    type Item;

    fn next(&self) -> Option<Self::Item>;
}

impl Iterator for i32 {
    type Item = i32;

    fn next(&self) -> Option<Self::Item> {
        Some(*self)
    }
}

struct Meters(u32);
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

trait Pilot {
    fn fly(&self) {}
}

trait Wizard {
    fn fly(&self) {}
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Fly for pilot")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Fly for wizard")
    }
}

impl Human {
    fn fly(&self) {
        println!("Fly for human")
    }
}

trait OutlinePrint: fmt::Display {
    fn print_outline(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len));
        println!("*{}*", "*".repeat(len - 2));
        println!("* {} *", output);
        println!("*{}*", "*".repeat(len - 2));
        println!("{}", "*".repeat(len));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {
    fn print_outline(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let mut p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };

    let p3 = p1 + p2;

    assert_eq!(p3, Point { x: 3, y: 3 });

    p1.x = 3;
    println!("{:?}", p1);

    let meters = Meters(1);
    let millimeters = Millimeters(1);

    assert_eq!((millimeters + meters).0, 1001);

    let human = Human {};

    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);

    p1.print_outline();

    let mut v = Wrapper(vec![
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
    ]);

    v.0.push(String::from("Felipe"));

    println!("Printing my vector of string, {}", v)
}
