use super::types::RedisDeserializationTypes;

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
/// use redis::modules::deserialize::deserialize_flat_command;
/// use redis::modules::types::RedisDeserializationTypes;
///
/// let mut command = "+OK\r\n";
/// let result = deserialize_flat_command(&mut command);
/// assert_eq!(result, RedisDeserializationTypes::SimpleString("OK".to_string()));
/// assert_eq!(command, "");
/// ```
pub fn deserialize_flat_command(command: &mut &str) -> RedisDeserializationTypes {
    if let Some(pos) = command.find("\r\n") {
        let simple_str = &command[..pos];
        let first_char = simple_str.chars().next().expect("Invalid Command");

        *command = &command[pos + 2..];

        match first_char {
            '+' => RedisDeserializationTypes::SimpleString(simple_str[1..].to_string()),
            '-' => RedisDeserializationTypes::ErrorMessage(simple_str[1..].to_string()),
            ':' => RedisDeserializationTypes::Integer(
                simple_str[1..].parse::<i64>().expect("To be a number"),
            ),
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
/// use redis::modules::deserialize::deserialize_bulk_string;
/// use redis::modules::types::RedisDeserializationTypes;
///
/// let mut command = "$6\r\nfoobar\r\n";
/// let result = deserialize_bulk_string(&mut command);
/// assert_eq!(result, Some(RedisDeserializationTypes::BulkString("foobar".to_string())));
/// assert_eq!(command, "");
/// ```
pub fn deserialize_bulk_string(command: &mut &str) -> Option<RedisDeserializationTypes> {
    if let Some(pos) = command.find("\r\n") {
        let start = pos + 2;

        if let Ok(count) = command[1..pos].parse::<usize>() {
            let string: String = command[start..].chars().take(count).collect();
            let len = string.len() + 2;

            if command.len() >= (start + len) {
                *command = &command[start + len..];

                return Some(RedisDeserializationTypes::BulkString(string));
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
/// use redis::modules::deserialize::deserialize_array;
/// use redis::modules::types::RedisDeserializationTypes;
///
/// let mut command = "*2\r\n+OK\r\n:1000\r\n";
/// let result = deserialize_array(&mut command);
/// assert_eq!(
///     result,
///     Some(vec![
///         RedisDeserializationTypes::SimpleString("OK".to_string()),
///         RedisDeserializationTypes::Integer(1000)
///     ])
/// );
/// assert_eq!(command, "");
/// ```
pub fn deserialize_array(command: &mut &str) -> Option<Vec<RedisDeserializationTypes>> {
    if let Some(pos) = command.find("\r\n") {
        let start = pos + 2;
        if let Ok(count) = command[1..pos].parse::<u32>() {
            let mut ret: Vec<RedisDeserializationTypes> = Vec::new();

            *command = &command[start..];
            for _ in 0..count {
                if let Some(result) = deserialize(command) {
                    ret.push(result);
                }
            }

            return Some(ret);
        } else {
            *command = &command[start..];
        }
    }

    None
}

/// Deserialize a command from the given string slice.
///
/// The command can be of various types, indicated by the first character:
/// - `$`: Bulk string
/// - `*`: Array
/// - `+`, `:`, `-`: Flat command (simple string, integer, or error)
///
/// # Arguments
///
/// * `command` - A mutable reference to a string slice containing the command.
///
/// # Returns
///
/// An `Option<Vec<Types>>` containing the deserialized command if successful, or `None` if the command is invalid.
///
/// # Example
///
/// ```
/// use redis::modules::deserialize::deserialize;
/// use redis::modules::types::RedisDeserializationTypes;
///
/// let mut command = "+OK\r\n";
/// let result = deserialize(&mut command);
/// assert_eq!(result, Some(RedisDeserializationTypes::SimpleString("OK".to_string())));
/// assert_eq!(command, "");
/// ```
pub fn deserialize(command: &mut &str) -> Option<RedisDeserializationTypes> {
    if let Some(first_char) = command.chars().next() {
        match first_char {
            '$' => {
                if let Some(result) = deserialize_bulk_string(command) {
                    return Some(result);
                }
            }
            '*' => {
                if let Some(result) = deserialize_array(command) {
                    return Some(RedisDeserializationTypes::Array(Box::new(result)));
                }
            }
            '+' | ':' | '-' => return Some(deserialize_flat_command(command)),
            _ => panic!("Invalid command"),
        };
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
        assert_eq!(
            result,
            RedisDeserializationTypes::SimpleString("OK".to_string())
        );
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_error_message() {
        let mut command = "-Error message\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(
            result,
            RedisDeserializationTypes::ErrorMessage("Error message".to_string())
        );
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_number_positive() {
        let mut command = ":+1000\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, RedisDeserializationTypes::Integer(1000));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_number_negative() {
        let mut command = ":-1000\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, RedisDeserializationTypes::Integer(-1000));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_number_0() {
        let mut command = ":0\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, RedisDeserializationTypes::Integer(0));
        assert_eq!(command, "");
    }

    #[test]
    #[should_panic(expected = "Invalid Command")]
    fn it_should_de_serialize_number_empty() {
        let mut command = "";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, RedisDeserializationTypes::Integer(0));
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_de_serialize_return_remaining_command() {
        let mut command = ":0\r\n$4\r\necho\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(result, RedisDeserializationTypes::Integer(0));
        assert_eq!(command, "$4\r\necho\r\n");
    }

    #[test]
    fn it_should_de_serialize_return_remaining_command_2() {
        let mut command = "+hello\r\n$4\r\necho\r\n+echo\r\n-Error Message\r\n";

        let result = deserialize_flat_command(&mut command);
        assert_eq!(
            result,
            RedisDeserializationTypes::SimpleString("hello".to_string())
        );
        assert_eq!(command, "$4\r\necho\r\n+echo\r\n-Error Message\r\n");
    }

    #[test]
    fn it_should_deserialize_bulk_string() {
        let mut command = "$4\r\nping\r\n";

        let result = deserialize_bulk_string(&mut command);
        assert_eq!(
            result,
            Some(RedisDeserializationTypes::BulkString("ping".to_string()))
        );
        assert_eq!(command, "");
    }

    #[test]
    fn it_should_deserialize_bulk_string_remaining_command() {
        let mut command = "$4\r\nping\r\n:123\r\n";

        let result = deserialize_bulk_string(&mut command);
        assert_eq!(
            result,
            Some(RedisDeserializationTypes::BulkString("ping".to_string()))
        );
        assert_eq!(command, ":123\r\n");
    }

    #[test]
    fn it_should_deserialize_bulk_string_remaining_command_2_with_special_chars() {
        let mut command = "$18\r\nping \r\nhello world\r\n$7\r\n1234567\r\n:4\r\n";

        let result = deserialize_bulk_string(&mut command);
        assert_eq!(
            result,
            Some(RedisDeserializationTypes::BulkString(
                "ping \r\nhello world".to_string()
            ))
        );
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
        assert_eq!(
            result,
            Some(RedisDeserializationTypes::BulkString("".to_string()))
        );
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
                RedisDeserializationTypes::SimpleString("echo".to_string()),
                RedisDeserializationTypes::Integer(11),
                RedisDeserializationTypes::BulkString("1234".to_string()),
                RedisDeserializationTypes::Array(Box::new(vec![
                    RedisDeserializationTypes::BulkString("1234".to_string()),
                    RedisDeserializationTypes::Integer(11)
                ])),
                RedisDeserializationTypes::BulkString("last".to_string())
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
                RedisDeserializationTypes::SimpleString("echo".to_string()),
                RedisDeserializationTypes::Integer(11),
                RedisDeserializationTypes::BulkString("1234".to_string()),
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
                RedisDeserializationTypes::BulkString("".to_string()),
                RedisDeserializationTypes::SimpleString("OK".to_string())
            ])
        );
        assert_eq!(command, "");
    }
}
