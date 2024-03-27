fn main() {
    let mut s = String::from("Hello world my name is felipe");
    let first_word: &str = first_word(&s);
    let mut copy_first_word = first_word.to_string();

    println!("{}, {}, {}", s, first_word, copy_first_word);
    copy_first_word.clear();
    println!("{}, {}, {}", s, first_word, copy_first_word);
    s.clear();
    println!("{}, {}", s, copy_first_word);
}

fn first_word(s: &String) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
