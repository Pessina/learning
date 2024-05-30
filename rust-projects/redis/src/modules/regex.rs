use regex::Regex;

pub fn is_array_pattern(s: &str) -> bool {
    let re = Regex::new(r"^\[\s*([a-zA-Z0-9 ]+,\s*)*[a-zA-Z0-9 ]*\s*\]$").unwrap();
    re.is_match(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn should_match_array() {
        let arr_str = "[a, b, c, d]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    #[ignore]
    fn should_match_array_lower_upper_case_and_space() {
        let arr_str = "[element1, Element2, element number 3, 3]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    #[ignore]
    fn should_match_array_empty() {
        let arr_str = "[]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    #[ignore]
    fn should_match_array_one_element() {
        let arr_str = "[1]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    #[ignore]
    fn should_match_array_big_array() {
        let arr_str =
            "[1, a, A, B, 123 sdlkjfsd fsd f, ksdjfds0928340921lfkdsjkfl lskjfds, aaaaaaaa]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    #[ignore]
    fn should_not_match_non_array_string() {
        let non_arr_str = "not an array";
        assert!(!is_array_pattern(non_arr_str))
    }

    #[test]
    #[ignore]
    fn should_not_match_malformed_array() {
        let malformed_arr_str = "[1, 2, 3";
        assert!(!is_array_pattern(malformed_arr_str))
    }

    #[test]
    #[ignore]
    fn should_not_match_array_with_invalid_characters() {
        let invalid_char_arr_str = "[1, 2, @, #]";
        assert!(!is_array_pattern(invalid_char_arr_str))
    }
}
