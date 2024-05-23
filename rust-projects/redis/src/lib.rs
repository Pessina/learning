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
    match command.chars().next() {
        Some('+') | Some('-') => {
            if let Some(pos) = command.find("\r\n") {
                let simple_str = &command[..pos];
                *command = &command[pos + 2..];
                Types::String(simple_str[1..].to_string())
            } else {
                panic!("Command not recognized")
            }
        }
        Some(':') => {
            if let Some(pos) = command.find("\r\n") {
                let simple_str = &command[..pos];
                *command = &command[pos + 2..];
                Types::Number(simple_str[1..].parse::<i64>().expect("To be a number"))
            } else {
                panic!("Command not recognized")
            }
        }
        _ => {
            panic!("Command not recognized")
        }
    }
}

// pub fn deserialize_bulk_string(command: &str) -> Option<String> {
//     match command {
//         "$-1\r\n" => None,
//         _ => {
//             let mut chars = command[1..].chars();
//             let count: String = chars.by_ref().take_while(|c| c.is_digit(10)).collect();

//             let ret = chars
//                 .by_ref()
//                 .skip(1)
//                 .take(count.parse::<usize>().expect("To be a number"))
//                 .collect::<String>();

//             Some(ret)
//         }
//     }
// }

// pub fn deserialize_array(command: &str) -> Option<Vec<Types>> {
//     match command {
//         "*-1\r\n" => None,
//         _ => {
//             let mut command_chars = command.chars();
//             let count: u64 = command_chars
//                 .by_ref()
//                 .skip(1)
//                 .take_while(|c| c.is_digit(10))
//                 .collect::<String>()
//                 .parse()
//                 .expect("To be a number");

//             println!("Count: {:?}", count);

//             let mut ret: Vec<Types> = Vec::new();

//             command_chars.by_ref().take(1).for_each(drop);

//             let mut t = command_chars.by_ref().collect::<String>();
//             for _ in 0..count {
//                 let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();

//                 let mut rest: &str = "";
//                 match remaining[0].chars().next().unwrap() {
//                     '+' => {
//                         let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();
//                         let current = remaining[0].to_string() + "\r\n";
//                         rest = remaining[1];

//                         let result = deserialize_simple_string(&current);
//                         ret.push(Types::String(result));
//                     }
//                     '-' => {
//                         let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();
//                         let current = remaining[0].to_string() + "\r\n";
//                         rest = remaining[1];

//                         let result = deserialize_error(&current);
//                         ret.push(Types::String(result));
//                     }
//                     ':' => {
//                         let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();
//                         let current = remaining[0].to_string() + "\r\n";
//                         rest = remaining[1];

//                         let result = deserialize_integer(&current);
//                         ret.push(Types::Number(result));
//                     }
//                     '$' => match deserialize_bulk_string(&t) {
//                         Some(result) => {
//                             let remaining: Vec<&str> = t.splitn(2, &result.to_string()).collect();
//                             rest = remaining[1].splitn(2, "\r\n").skip(1).next().unwrap();

//                             println!("{:?}", rest);

//                             ret.push(Types::String(result));
//                         }
//                         None => {}
//                     },
//                     '*' => match deserialize_array(&t) {
//                         Some(result) => {
//                             rest = &t;
//                             ret.push(Types::Array(Box::new(result)))
//                         }
//                         None => {}
//                     },
//                     _ => {}
//                 }

//                 println!("Current ret: {:?}", ret);

//                 t = rest.chars().collect();
//             }

//             Some(ret)
//         }
//     }
// }

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

    // #[test]
    // fn it_should_deserialize_bulk_string() {
    //     let test_str = "ping";

    //     let result = deserialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
    //     assert_eq!(result, Some(test_str.to_string()));
    // }

    // #[test]
    // fn it_should_deserialize_bulk_string_0_elements() {
    //     let test_str = "";

    //     let result = deserialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
    //     assert_eq!(result, Some(test_str.to_string()));
    // }

    // #[test]
    // fn it_should_deserialize_bulk_string_many_elements() {
    //     let test_str = "lskdfjkldsjf\n\r\n skjdhfjkdshf ";

    //     let result = deserialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
    //     assert_eq!(result, Some(test_str.to_string()));
    // }

    // #[test]
    // fn it_should_deserialize_bulk_string_null_elements() {
    //     let result = deserialize_bulk_string("$-1\r\n");
    //     assert_eq!(result, None);
    // }

    // #[test]
    // fn it_should_deserialize_array() {
    //     deserialize_array(
    //         "*4\r\n+echo\r\n:11\r\n$4\r\n1234\r\n*2\r\n$4\r\n1234\r\n:11\r\n$4\r\nlast\r\n",
    //     );
    // }
}
