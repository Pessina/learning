fn main() {
    let mut count = 0;

    println!("Loop");

    'outer_loop: loop {
        let mut remaining = 10;
        let a = 'inner_loop: loop {
            if remaining == 9 {
                break 'inner_loop 3;
            }

            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        };

        println!("{remaining}");
        println!("{a}");

        count += 1;
    }

    let mut count = 3;

    println!("While");

    while count > 0 {
        println!("{count}");
        count -= 1;
    }

    println!("For");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}", element);
    }

    println!("For with range");

    for i in (0..=10).rev() {
        println!("{}", i);
    }
}
