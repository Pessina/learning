pub mod exercise_1;

fn main() {
    let mut vec = vec![
        1, 5, 7, 8, 8, 9, 1, 3, 5, 7, 9, 9, 4, 3, 38, 2, 7, 42, 6, 83, 2, 5,
    ];

    let median = exercise_1::median(&mut vec);
    println!("Median: {median}");

    let mode = exercise_1::mode(&mut vec);
    println!("Mode: {mode}");
}
