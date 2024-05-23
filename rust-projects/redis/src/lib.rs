#[derive(Debug)]
pub enum Types {
    String(String),
    Number(i64),
}

// pub fn deserialize_command(command: Vec<&str>) {}

pub fn deserialize_simple_string(command: &str) -> String {
    command[1..command.len() - 2].to_string()
}

pub fn deserialize_bulk_string(command: &str) -> Option<String> {
    match command {
        "$-1\r\n" => None,
        _ => {
            let mut chars = command[1..].chars();
            let count: String = chars.by_ref().take_while(|c| c.is_digit(10)).collect();

            let ret = chars
                .by_ref()
                .skip(1)
                .take(count.parse::<usize>().expect("To be a number"))
                .collect::<String>();

            Some(ret)
        }
    }
}

pub fn deserialize_integer(command: &str) -> i64 {
    command[1..command.len() - 2]
        .parse()
        .expect("To be a 64 bit number")
}

pub fn deserialize_error(command: &str) -> String {
    command[1..command.len() - 2].to_string()
}

pub fn deserialize_array(command: &str) -> Option<Vec<Types>> {
    match command {
        "*-1\r\n" => None,
        _ => {
            let mut command_chars = command.chars();
            let count: u64 = command_chars
                .by_ref()
                .skip(1)
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .expect("To be a number");

            println!("Count: {:?}", count);

            let mut ret: Vec<Types> = Vec::new();

            command_chars.by_ref().take(1).for_each(drop);

            let mut t = command_chars.by_ref().collect::<String>();
            for _ in 0..count {
                let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();

                let mut rest: &str = "";
                match remaining[0].chars().next().unwrap() {
                    '+' => {
                        let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();
                        let current = remaining[0].to_string() + "\r\n";
                        rest = remaining[1];

                        let result = deserialize_simple_string(&current);
                        ret.push(Types::String(result));
                    }
                    '-' => {
                        let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();
                        let current = remaining[0].to_string() + "\r\n";
                        rest = remaining[1];

                        let result = deserialize_error(&current);
                        ret.push(Types::String(result));
                    }
                    ':' => {
                        let remaining: Vec<&str> = t.splitn(2, "\r\n").collect();
                        let current = remaining[0].to_string() + "\r\n";
                        rest = remaining[1];

                        let result = deserialize_integer(&current);
                        ret.push(Types::Number(result));
                    }
                    '$' => {
                        let remaining: Vec<&str> = t.splitn(3, "\r\n").collect();
                        let current = format!("{}{}{}", remaining[0], "\r\n", remaining[1]);
                        rest = remaining[2];

                        match deserialize_bulk_string(&current) {
                            Some(result) => ret.push(Types::String(result)),
                            None => {}
                        }
                    }
                    // '*' => {
                    //     let result = deserialize_array(&current);
                    //     ret.push(Types::String(result));
                    // }
                    _ => {}
                }

                println!("Current ret: {:?}", ret);

                t = rest.chars().collect();
            }

            Some(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_simple_string() {
        let test_str = "OK";

        let result = deserialize_simple_string(&format!("+{}\r\n", test_str));
        assert_eq!(result, test_str);
    }

    #[test]
    fn it_should_deserialize_bulk_string() {
        let test_str = "ping";

        let result = deserialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
        assert_eq!(result, Some(test_str.to_string()));
    }

    #[test]
    fn it_should_deserialize_bulk_string_0_elements() {
        let test_str = "";

        let result = deserialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
        assert_eq!(result, Some(test_str.to_string()));
    }

    #[test]
    fn it_should_deserialize_bulk_string_many_elements() {
        let test_str = "lskdfjkldsjf\n\r\n skjdhfjkdshf ";

        let result = deserialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
        assert_eq!(result, Some(test_str.to_string()));
    }

    #[test]
    fn it_should_deserialize_bulk_string_null_elements() {
        let result = deserialize_bulk_string("$-1\r\n");
        assert_eq!(result, None);
    }

    #[test]
    fn it_should_deserialize_integer_positive() {
        let result = deserialize_integer(":+64\r\n");
        assert_eq!(result, 64)
    }

    #[test]
    fn it_should_deserialize_integer_negative() {
        let result = deserialize_integer(":-64\r\n");
        assert_eq!(result, -64)
    }

    #[test]
    fn it_should_deserialize_integer_0() {
        let result = deserialize_integer(":0\r\n");
        assert_eq!(result, 0)
    }

    #[test]
    fn it_should_deserialize_error() {
        let result = deserialize_error("-Error message\r\n");
        assert_eq!(result, "Error message")
    }

    #[test]
    #[ignore]
    fn it_should_deserialize_array() {
        deserialize_array("*3\r\n+echo\r\n:11\r\n$4\r\n1234\r\n");
    }
}
