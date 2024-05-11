fn print_point((x, y): (i32, i32)) {
    println!("{}, {}", x, y)
}

fn main() {
    let mut point = (32, 32);
    print_point(point);

    point.0 = 3;

    println!("{}, {}", point.0, point.1);

    let x = 10;

    match x {
        2 | 5 => println!("Matched 2 | 5"),
        10 => println!("Mathced {x}"),
        _ => println!("Nothing matched"),
    }
}
