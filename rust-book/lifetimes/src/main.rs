use std::fmt::Display;

struct Point<'a> {
    target: &'a str,
}

fn main() {
    let str_1 = String::from("123");
    let long_result: &str;
    let long_result_2: &str;
    let short_result: &str;
    let str_3: &'static str = "123";

    {
        let str_2 = String::from("abcd");
        long_result = longest(str_1.as_str(), str_2.as_str());
        long_result_2 = longest_with_announcement(str_1.as_str(), str_2.as_str(), "Olá pessoal");
        short_result = shortest(str_1.as_str(), str_2.as_str());
        println!("{}", long_result);
        println!("{}", long_result_2);
    }

    println!("{}", short_result);

    let my_str = String::from("123.321");
    let my_sub_str = my_str.split(".").next().expect("String doesn't have .");

    let point = Point { target: my_sub_str };

    println!("{}", point.target);
    println!("{}", str_3);
}

fn longest<'a>(str_1: &'a str, str_2: &'a str) -> &'a str {
    if str_1.len() > str_2.len() {
        str_1
    } else {
        str_2
    }
}

fn shortest<'a>(str_1: &'a str, str_2: &str) -> &'a str {
    if str_1.len() < str_2.len() {
        str_1
    } else {
        str_1
    }
}

fn longest_with_announcement<'a, T>(str_1: &'a str, str_2: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    print!("{}", ann);
    if str_1.len() > str_2.len() {
        str_1
    } else {
        str_2
    }
}
