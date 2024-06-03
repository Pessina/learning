use regex::Regex;

#[derive(PartialEq, Eq)]
pub enum ArrayPlacement {
    LEFT,
    RIGHT,
}

pub fn is_array_pattern(s: &str) -> bool {
    let re = Regex::new(r"^\[\s*([a-zA-Z0-9 ]+,\s*)*[a-zA-Z0-9 ]*\s*\]$").unwrap();
    re.is_match(s)
}

pub fn insert_on_array(s: &str, value: &str, placement: ArrayPlacement) -> Result<String, String> {
    if is_array_pattern(s) {
        if value == "" {
            return Ok(s.to_string());
        }

        let s = s.trim();
        let mut s: Vec<&str> = s[1..s.len() - 1]
            .split(",")
            .filter(|v| v.len() > 0)
            .collect();

        if placement == ArrayPlacement::LEFT {
            s.push(value);
        } else {
            s.insert(0, value)
        }

        Ok(format!("[{}]", s.join(",")))
    } else {
        Err("The string it's not an array".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_match_array() {
        let arr_str = "[a, b, c, d]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    fn should_match_array_lower_upper_case_and_space() {
        let arr_str = "[element1, Element2, element number 3, 3]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    fn should_match_array_empty() {
        let arr_str = "[]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    fn should_match_array_one_element() {
        let arr_str = "[1]";
        assert!(is_array_pattern(arr_str))
    }

    #[test]
    fn should_not_match_non_array_string() {
        let non_arr_str = "not an array";
        assert!(!is_array_pattern(non_arr_str))
    }

    #[test]
    fn should_not_match_malformed_array() {
        let malformed_arr_str = "[1, 2, 3";
        assert!(!is_array_pattern(malformed_arr_str))
    }

    #[test]
    fn should_not_match_array_with_invalid_characters() {
        let invalid_char_arr_str = "[1, 2, @, #]";
        assert!(!is_array_pattern(invalid_char_arr_str))
    }

    #[test]
    fn should_insert_on_array_4_elements() {
        let array = "[1,2,3]";

        let result = insert_on_array(array, "4", ArrayPlacement::LEFT);

        assert_eq!("[1,2,3,4]", result.unwrap())
    }

    #[test]
    fn should_not_insert_on_array_right() {
        let array = "[element]";

        let result = insert_on_array(array, "3", ArrayPlacement::RIGHT);

        assert_eq!("[3,element]", result.unwrap())
    }

    #[test]
    fn should_not_insert_on_array_right_multiple_times() {
        let array = "[element]";

        let result = insert_on_array(array, "3", ArrayPlacement::RIGHT);
        let result = insert_on_array(&result.unwrap(), "4", ArrayPlacement::RIGHT);
        let result = insert_on_array(&result.unwrap(), "5", ArrayPlacement::RIGHT);

        assert_eq!("[5,4,3,element]", result.unwrap())
    }

    #[test]
    fn should_insert_on_array_empty() {
        let array = "[]";

        let result = insert_on_array(array, "my name", ArrayPlacement::LEFT);

        assert_eq!("[my name]", result.unwrap())
    }

    #[test]
    fn should_error_on_insert_on_array_empty_string() {
        let array = "";

        let result = insert_on_array(array, "my name", ArrayPlacement::LEFT);

        assert!(result.is_err())
    }

    #[test]
    fn should_error_on_insert_on_array_2_string() {
        let array = "my name";

        let result = insert_on_array(array, "my name", ArrayPlacement::LEFT);

        assert!(result.is_err())
    }

    #[test]
    fn should_error_on_insert_on_array_multiple_times() {
        let array = "[element]";

        let result = insert_on_array(array, "my name", ArrayPlacement::LEFT);
        let result = insert_on_array(&result.unwrap(), "3", ArrayPlacement::LEFT);
        let result = insert_on_array(&result.unwrap(), "", ArrayPlacement::LEFT);

        assert_eq!("[element,my name,3]", result.unwrap())
    }

    #[test]
    fn should_not_insert_on_array_empty_element() {
        let array = "[]";

        let result = insert_on_array(array, "", ArrayPlacement::LEFT);

        assert_eq!("[]", result.unwrap())
    }
}
