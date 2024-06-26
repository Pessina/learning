use std::collections::HashMap;

pub mod exercise_1;
pub mod exercise_2;
pub mod exercise_3;

fn main() {
    let mut vec = vec![
        1, 5, 7, 8, 8, 9, 1, 3, 5, 7, 9, 9, 4, 3, 38, 2, 7, 42, 6, 83, 2, 5,
    ];

    let median = exercise_1::median(&mut vec);
    println!("Median: {}", median);

    let mode = exercise_1::mode(&mut vec);
    println!("Mode: {}", mode);

    let pig_latin = exercise_2::to_pig_latin(String::from("aelipe"));
    println!("Pig Latin: {}", pig_latin);

    let mut company = exercise_3::Company {
        departments: HashMap::<String, Vec<String>>::new(),
    };

    company.add("Felipe", "IT");
    company.add("Pedro", "IT");
    company.add("Carlos", "IT");
    company.add("Cristina", "RH");
    company.add("Marcos", "Sales");
    company.add("Lucas", "Sale");

    let it = company.get_by_department("IT").unwrap();
    let sales = company.get_by_department("Sales").unwrap();
    let invalid = company.get_by_department("CTO");

    let all = company.get_all();

    println!("IT Department:");
    println!("{}", it.join(", "));

    println!("Sales Department:");
    println!("{}", sales.join(", "));

    println!("Invalid Department:");
    match invalid {
        Some(value) => {
            println!("{}", value.join(", "));
        }
        None => println!("None"),
    }

    println!("All Employees:");
    println!("{}", all.join(", "));
}
