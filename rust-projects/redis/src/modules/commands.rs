use super::types::RedisDeserializationTypes;

const INVALID_COMMAND: &'static str = "-Invalid Command\r\n";

pub fn execute_command(command: &RedisDeserializationTypes) -> String {
    match command {
        RedisDeserializationTypes::Array(ref a) => match a[0] {
            RedisDeserializationTypes::BulkString(ref s) => match s.as_ref() {
                "PING" => return "+PONG\r\n".to_string(),
                "ECHO" => {
                    if a.len() == 2 {
                        if let RedisDeserializationTypes::BulkString(ref echo) = a[1] {
                            return format!("+{}\r\n", echo);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }

    INVALID_COMMAND.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_ping_pong() {
        let response = execute_command(&RedisDeserializationTypes::Array(Box::new(vec![
            RedisDeserializationTypes::BulkString("PING".to_string()),
        ])));
        assert_eq!(response, "+PONG\r\n")
    }

    #[test]
    fn it_should_echo() {
        let response = execute_command(&RedisDeserializationTypes::Array(Box::new(vec![
            RedisDeserializationTypes::BulkString("ECHO".to_string()),
            RedisDeserializationTypes::BulkString("Hello World".to_string()),
        ])));
        assert_eq!(response, "+Hello World\r\n")
    }

    #[test]
    fn it_should_error_echo() {
        let response = execute_command(&RedisDeserializationTypes::Array(Box::new(vec![
            RedisDeserializationTypes::BulkString("ECHO".to_string()),
        ])));
        assert_eq!(response, "-Invalid Command\r\n")
    }

    #[test]
    fn it_should_error() {
        let response = execute_command(&RedisDeserializationTypes::Array(Box::new(vec![
            RedisDeserializationTypes::BulkString("123".to_string()),
            RedisDeserializationTypes::BulkString("Hello World".to_string()),
        ])));
        assert_eq!(response, "-Invalid Command\r\n")
    }
}
