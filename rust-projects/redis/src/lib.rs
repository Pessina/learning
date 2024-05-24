#[derive(Debug, PartialEq)]
pub enum Types {
    String(String),
    Number(i64),
    Array(Box<Vec<Types>>),
}

/// Deserialize a flat command from the given string slice.
///
/// Flat commands are prefixed with `+`, `-`, or `:` and terminated by `\r\n`.
///
/// # Arguments
///
/// * `command` - A mutable reference to a string slice containing the command.
///
/// # Returns
///
/// A `String` containing the deserialized command without the prefix and termination.
///
/// # Example
///
/// ```
/// use redis::{deserialize_flat_command, Types};
///
/// let mut command = "+OK\r\n";
/// let result = deserialize_flat_command(&mut command);
/// assert_eq!(result, Types::String("OK".to_string()));
/// assert_eq!(command, "");
/// ```
pub fn deserialize_flat_command(command: &mut &str) -> Types {
    if let Some(pos) = command.find("\r\n") {
        let simple_str = &command[..pos];
        let first_char = simple_str.chars().next().expect("Invalid Command");

        *command = &command[pos + 2..];

        match first_char {
            '+' | '-' => Types::String(simple_str[1..].to_string()),
            ':' => Types::Number(simple_str[1..].parse::<i64>().expect("To be a number")),
            _ => panic!("Invalid Command"),
        }
    } else {
        panic!("Invalid Command")
    }
}

/// Deserialize a bulk string from the given string slice.
///
/// Bulk strings are prefixed with `$` followed by the length of the string and terminated by `\r\n`.
///
/// # Arguments
///
/// * `command` - A mutable reference to a string slice containing the command.
///
/// # Returns
///
/// An `Option<String>` containing the deserialized bulk string if successful, or `None` if the bulk string is null.
///
/// # Example
///
/// ```
/// use redis::deserialize_bulk_string;
///
/// let mut command = "$6\r\nfoobar\r\n";
/// let result = deserialize_bulk_string(&mut command);
/// assert_eq!(result, Some("foobar".to_string()));
/// assert_eq!(command, "");
/// ```
pub fn deserialize_bulk_string(command: &mut &str) -> Option<String> {
    if let Some(pos) = command.find("\r\n") {
        let start = pos + 2;

        if let Ok(count) = command[1..pos].parse::<usize>() {
            let string: String = command[start..].chars().take(count).collect();
            let len = string.len() + 2;

            if command.len() >= (start + len) {
                *command = &command[start + len..];

                return Some(string);
            }
        } else {
            *command = &command[start..];
            return None;
        }
    }

    panic!("Invalid Command")
}

/// Deserialize an array from the given string slice.
///
/// Arrays are prefixed with `*` followed by the number of elements in the array and terminated by `\r\n`.
///
/// # Arguments
///
/// * `command` - A mutable reference to a string slice containing the command.
///
/// # Returns
///
/// An `Option<Vec<Types>>` containing the deserialized array if successful, or `None` if the array is null.
///
/// # Example
///
/// ```
/// use redis::{deserialize_array, Types};
///
/// let mut command = "*2\r\n+OK\r\n:1000\r\n";
/// let result = deserialize_array(&mut command);
/// assert_eq!(
///     result,
///     Some(vec![
///         Types::String("OK".to_string()),
///         Types::Number(1000)
///     ])
/// );
/// assert_eq!(command, "");
/// ```
pub fn deserialize_array(command: &mut &str) -> Option<Vec<Types>> {
    if let Some(pos) = command.find("\r\n") {
        let start = pos + 2;
        if let Ok(count) = command[1..pos].parse::<u32>() {
            let mut ret: Vec<Types> = Vec::new();

            *command = &command[start..];
            for _ in 0..count {
                if let Some(first_char) = command.chars().next() {
                    match first_char {
                        '$' => {
                            if let Some(result) = deserialize_bulk_string(command) {
                                ret.push(Types::String(result))
                            }
                        }
                        '*' => {
                            if let Some(result) = deserialize_array(command) {
                                ret.push(Types::Array(Box::new(result)))
                            }
                        }
                        '+' | ':' | '-' => ret.push(deserialize_flat_command(command)),
                        _ => panic!("Invalid command"),
                    }
                }
            }

            return Some(ret);
        } else {
            *command = &command[start..];
        }
    }

    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_de_serialize_simple_string() {
        let mut command = "+OK\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::String("OK".to_string()));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_error_message() {
        let mut command = "-Error message\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::String("Error message".to_string()));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_number_positive() {
        let mut command = ":+1000\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::Number(1000));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_number_negative() {
        let mut command = ":-1000\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::Number(-1000));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_number_0() {
        let mut command = ":0\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::Number(0));
        assert_eq!(command, "");
    }

    #[test]
    #[should_panic(expected = "Invalid Command")]
    fn it_should_de_serialize_number_empty() {
        let mut command = "";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::Number(0));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_return_remaining_command() {
        let mut command = ":0\r\n$4\r\necho\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::Number(0));
        assert_eq!(command, "$4\r\necho\r\n");
    }

    #[test]
    fn it_should_de_serialize_return_remaining_command_2() {
        let mut command = "+hello\r\n$4\r\necho\r\n+echo\r\n-Error Message\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, Types::String("hello".to_string()));
        assert_eq!(command, "$4\r\necho\r\n+echo\r\n-Error Message\r\n");
    }

    #[test]
    fn it_should_deserialize_bulk_string() {
        let mut command = "$4\r\nping\r\n";

        let result = deserialize_bulk_string(&mut command);
        assert_eq!(result, Some("ping".to_string()));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_bulk_string_remaining_command() {
        let mut command = "$4\r\nping\r\n:123\r\n";

        let result = deserialize_bulk_string(&mut command);
        assert_eq!(result, Some("ping".to_string()));
        assert_eq!(command, ":123\r\n");
    }

    #[test]
    fn it_should_deserialize_bulk_string_remaining_command_2_with_special_chars() {
        let mut command = "$18\r\nping \r\nhello world\r\n$7\r\n1234567\r\n:4\r\n";

        let result = deserialize_bulk_string(&mut command);
        assert_eq!(result, Some("ping \r\nhello world".to_string()));
        assert_eq!(command, "$7\r\n1234567\r\n:4\r\n");
    }

    #[test]
    fn it_should_deserialize_bulk_string_null_elements() {
        let mut command = "$-1\r\n";
        let result = deserialize_bulk_string(&mut command);
        assert_eq!(result, None);
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_bulk_string_empty() {
        let mut command = "$0\r\n\r\n";
        let result = deserialize_bulk_string(&mut command);
        assert_eq!(result, Some("".to_string()));
        assert_eq!(command, "");
    }

    #[test]
    #[should_panic(expected = "Invalid Command")]
    fn it_should_deserialize_bulk_string_incomplete() {
        let mut command = "$4\r\npin";
        let result = deserialize_bulk_string(&mut command);
        assert_eq!(result, None);
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_array_nested_arr() {
        let mut command =
            "*5\r\n+echo\r\n:11\r\n$4\r\n1234\r\n*2\r\n$4\r\n1234\r\n:11\r\n$4\r\nlast\r\n";
        let result = deserialize_array(&mut command);
        assert_eq!(
            result,
            Some(Vec::from([
                Types::String("echo".to_string()),
                Types::Number(11),
                Types::String("1234".to_string()),
                Types::Array(Box::new(vec![
                    Types::String("1234".to_string()),
                    Types::Number(11)
                ])),
                Types::String("last".to_string())
            ]))
        );
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_array() {
        let mut command = "*4\r\n+echo\r\n:11\r\n$4\r\n1234\r\n";
        let result = deserialize_array(&mut command);
        assert_eq!(
            result,
            Some(vec![
                Types::String("echo".to_string()),
                Types::Number(11),
                Types::String("1234".to_string()),
            ])
        );
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_array_0_elements() {
        let mut command = "*0\r\n";
        let result = deserialize_array(&mut command);
        assert_eq!(result, Some(Vec::new()));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_array_null() {
        let mut command = "*-1\r\n";
        let result = deserialize_array(&mut command);
        assert_eq!(result, None);
        assert_eq!(command, "");
    }

    #[test]
    #[should_panic(expected = "Invalid Command")]
    fn it_should_deserialize_array_incomplete() {
        let mut command = "*2\r\n+echo\r\n:11";
        let result = deserialize_array(&mut command);
        assert_eq!(result, None);
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_array_with_empty_bulk_string() {
        let mut command = "*2\r\n$0\r\n\r\n+OK\r\n";
        let result = deserialize_array(&mut command);
        assert_eq!(
            result,
            Some(vec![
                Types::String("".to_string()),
                Types::String("OK".to_string())
            ])
        );
        assert_eq!(command, "");
    }
}
