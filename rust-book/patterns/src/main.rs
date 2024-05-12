struct Point {
    x: i32,
    y: i32,
}

enum Enum {
    Base,
    Point { x: i32, y: i32 },
    Text(String),
    Number(i32),
}

fn print_point((x, y): (i32, i32)) {
    println!("{}, {}", x, y)
}

fn ignore_function(_: i32, y: i32) {
    println!("This is the ignore function{}", y);
}

fn main() {
    let mut point = (32, 32);
    print_point(point);

    point.0 = 3;

    println!("{}, {}", point.0, point.1);

    let x = 10;

    match x {
        2 | 5 => println!("Matched 2 | 5"),
        5..=11 => println!("Matched {x}"),
        _ => println!("Nothing matched"),
    }

    let y = 'b';

    match y {
        'a'..='f' => println!("Early ASCII"),
        'g'..='z' => println!("Late ASCII"),
        _ => println!("Nothing matched"),
    }

    let p = Point { x: 2, y: 3 };

    let Point { x: a, y: b } = p;
    println!("Point, {a}, {b}");

    match p {
        Point { x: 22, y } => println!("X is 22"),
        Point { x, y: 3 } => println!("Y is 3"),
        Point { x, y } => println!("No matches"),
    }

    let my_enum_base = Enum::Base;
    let my_enum_point = Enum::Point { x: 3, y: 4 };
    let my_enum_text = Enum::Text(String::from("Testing my code"));
    let my_enum_number = Enum::Number(10);

    match my_enum_text {
        Enum::Base => println!("Base"),
        Enum::Point { x, y } => print!("My point {x}, {y}"),
        Enum::Text(s) => println!("My string {s}"),
        Enum::Number(x) => println!("My number {x}"),
    }

    ignore_function(3, 4);

    let t = (1, 2, 3, 4, 5);
    match t {
        (first, _, third, _, fifth) => println!("Printing the value of t {x}"),
    }

    let _x = 5;
    let y = 3;

    println!("_x and y values: {_x}, {y}");

    let num = Some(5);
    let needle = 5;

    match num {
        Some(x) if x == needle => println!("Found the needle"),
        Some(x) if x % 2 == 0 => println!("The number is par"),
        Some(x) => println!("Not needle not par"),
        None => println!("This isn't a number"),
    }

    let x = 5;
    let y = true;

    match x {
        4 | 6 | 8 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 4 };

    match msg {
        Message::Hello {
            id: message_id @ 0..=5,
        } => println!("Message id found in rage 0 to 5 {message_id}"),
        Message::Hello { id: 6..=30 } => println!("Message id found in rage 6, 30"),
        Message::Hello { id } => {
            println!("Message id: {id}")
        }
    }

    let a = [(0, 1)];
}
