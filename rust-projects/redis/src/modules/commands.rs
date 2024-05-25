use super::types::RedisDeserializationTypes;

const INVALID_COMMAND: &'static str = "-Invalid Command\r\n";

pub fn execute_command(command: &RedisDeserializationTypes) -> String {
    match command {
        RedisDeserializationTypes::Array(ref a) => match a[0] {
            RedisDeserializationTypes::BulkString(ref s) => match s.as_ref() {
                "PING" => return "+PONG\r\n".to_string(),
                "ECHO" => {
                    if let RedisDeserializationTypes::BulkString(ref echo) = a[1] {
                        return format!("+{}\r\n", echo.to_string());
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
