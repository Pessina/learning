fn main() {
    println!("Hello, world!");

    let y = {
        let x = 4;
        x + 2
    };

    let y = another_function(y);
    println!("y: {y}");
}

fn another_function(x: i32) -> i32 {
    println!("Test second function {x}");
    x + 10
}
