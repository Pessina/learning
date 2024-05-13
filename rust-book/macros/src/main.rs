use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! create_custom_vec {
    ($( $x: expr ), *) => {
        {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
            temp_vec.push($x);
        )*
        temp_vec
    }
    };
}

fn main() {
    let v = create_custom_vec![1, 2, 3, 4, 5];

    for i in v {
        print!("{i}, ");
    }

    println!();

    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();
}
