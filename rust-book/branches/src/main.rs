fn main() {
    let number = 5;

    let sentence = if number < 5 {
        "Number is less than 5"
    } else if number == 5 {
        "Number is 5"
    } else {
        "Number is greater than 5"
    };

    println!("{sentence}")
}
