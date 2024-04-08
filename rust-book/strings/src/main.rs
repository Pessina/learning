fn main() {
    let s = String::new();

    let mut s1 = "My name".to_string();

    s1.push(' ');

    let s2 = String::from("is Felipe");

    s1.push_str(&s2);

    println!("s1: {s1}");

    let mut s3 = String::from("Hello");
    let mut s4 = String::from("World");

    // let g = &s4[..]; // De-ref coercion

    s3 = s3 + " " + &s4;
    s4 = "New value for 24".to_string();

    println!("s3: {s3}");
    println!("s4: {s4}");

    let s5 = format!("{s1}-{s3}-{s4}");

    println!("{s5}");

    println!("Chars:");

    for c in s5.chars() {
        println!("{c}")
    }

    println!("Bytes:");

    for c in s5.bytes() {
        println!("{c}")
    }
}
