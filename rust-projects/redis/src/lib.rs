// pub fn de_serialize_command(command: Vec<&str>) {}

pub fn de_serialize_simple_string(command: &str) -> String {
    command[1..command.len() - 2].to_string()
}

pub fn de_serialize_bulk_string(command: &str) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_de_serialize_simple_string() {
        let test_str = "OK";

        let result = de_serialize_simple_string(&format!("+{}\r\n", test_str));
        assert_eq!(result, test_str);
    }

    #[test]
    fn it_should_de_serialize_bulk_string() {
        let test_str = "ping";

        let result = de_serialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
        assert_eq!(result, Some(test_str.to_string()));
    }

    #[test]
    fn it_should_de_serialize_bulk_string_0_elements() {
        let test_str = "";

        let result = de_serialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
        assert_eq!(result, Some(test_str.to_string()));
    }

    #[test]
    fn it_should_de_serialize_bulk_string_many_elements() {
        let test_str = "lskdfjkldsjf\n\r\n skjdhfjkdshf ";

        let result = de_serialize_bulk_string(&format!("${}\r\n{}\r\n", test_str.len(), test_str));
        assert_eq!(result, Some(test_str.to_string()));
    }

    #[test]
    fn it_should_de_serialize_bulk_string_null_elements() {
        let result = de_serialize_bulk_string("$-1\r\n");
        assert_eq!(result, None);
    }
}
