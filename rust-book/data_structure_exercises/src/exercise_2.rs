pub fn to_pig_latin(word: String) -> String {
    let first_char = word.chars().next();

    let is_vowel = match first_char {
        Some(value) => match value.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        },
        None => false,
    };

    if is_vowel {
        word + "-hay"
    } else {
        let ret: String = word.chars().skip(1).take(word.len() - 1).collect();
        let unwrapped_first_char = first_char.unwrap();
        (format!("{ret}-{unwrapped_first_char}ay")).to_string()
    }
}
