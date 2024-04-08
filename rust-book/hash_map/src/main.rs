use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert("Felipe", 26);
    map.insert("Isabela", 24);
    map.entry("Carlos").or_insert(56);

    let felipe_age = map.get("Felipe").copied().unwrap_or(0);
    let pedro_age = map.get("Pedro").copied().unwrap_or(0);

    println!("Felipe: {felipe_age}");
    println!("Pedro: {pedro_age}");

    for (key, value) in map {
        println!("{key} and {value}")
    }

    let mut map_string: HashMap<i32, String> = HashMap::new();

    let mut isabela = String::from("isabela");

    map_string.insert(26, String::from("Felipe"));
    map_string.insert(24, isabela);

    // isabela.push('s'); // Invalid operation, the has owns the data

    let mut words_map = HashMap::new();

    let demo_string = String::from("hello world wonderful world");

    for word in demo_string.split_whitespace() {
        let count = words_map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in words_map {
        println!("{key}: {value}");
    }

    // println!("{map}");
}
