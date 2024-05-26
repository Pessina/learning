use std::sync::{Arc, Mutex};

use chrono::{Duration, TimeZone, Utc};

use super::{
    store::{Redis, RedisCell},
    types::RedisDeserializationTypes,
};

const INVALID_COMMAND: &'static str = "-Invalid Command\r\n";

pub fn execute_command(command: &RedisDeserializationTypes, redis: Arc<Mutex<Redis>>) -> String {
    match command {
        RedisDeserializationTypes::Array(a) => match a[0] {
            RedisDeserializationTypes::BulkString(ref s) => match s.as_ref() {
                "PING" => return "+PONG\r\n".to_string(),
                "ECHO" => {
                    if a.len() == 2 {
                        if let RedisDeserializationTypes::BulkString(ref echo) = a[1] {
                            return format!("+{}\r\n", echo);
                        }
                    }
                }
                "SET" => {
                    if a.len() == 3 {
                        if let [_, RedisDeserializationTypes::BulkString(ref key), RedisDeserializationTypes::BulkString(ref value)] =
                            a[..]
                        {
                            redis.lock().unwrap().set(
                                key.to_string(),
                                RedisCell {
                                    value: value.to_string(),
                                    expiry: None,
                                },
                            );
                            return "+OK\r\n".to_string();
                        }
                    }
                    if a.len() == 5 {
                        if let [_, RedisDeserializationTypes::BulkString(ref key), RedisDeserializationTypes::BulkString(ref value), RedisDeserializationTypes::BulkString(ref expiry_config), RedisDeserializationTypes::Integer(ref expiry_value)] =
                            a[..]
                        {
                            let expiry = match expiry_config.as_ref() {
                                "EX" => Utc::now() + Duration::seconds(*expiry_value),
                                "PX" => Utc::now() + Duration::milliseconds(*expiry_value),
                                "EAXT" => Utc.timestamp_opt(*expiry_value as i64, 0).unwrap(),
                                "PXAT" => {
                                    Utc.timestamp_opt((*expiry_value as i64) / 1000, 0).unwrap()
                                }
                                _ => panic!("Invalid Command"),
                            };

                            redis.lock().unwrap().set(
                                key.to_string(),
                                RedisCell {
                                    value: value.to_string(),
                                    expiry: Some(expiry),
                                },
                            );
                            return "+OK\r\n".to_string();
                        }
                    }
                }
                "GET" => {
                    if a.len() == 2 {
                        if let [_, RedisDeserializationTypes::BulkString(ref key)] = a[..] {
                            match redis.lock().unwrap().get(key) {
                                Some(result) => return format!("+{}\r\n", result.value),
                                None => return format!("+NONE\r\n"),
                            }
                        }
                    }
                }
                // Mock config, to bypass redis-benchmark request
                "CONFIG" => {
                    return "*2\r\n$4\r\nsave\r\n$23\r\n3600 1 300 100 60 10000\r\n*2\r\n$10\r\nappendonly\r\n$2\r\nno\r\n"
                        .to_string();
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

    struct Setup {
        redis: Arc<Mutex<Redis>>,
    }

    fn setup() -> Setup {
        Setup {
            redis: Arc::new(Mutex::new(Redis::new())),
        }
    }

    #[test]
    fn it_should_ping_pong() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("PING".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+PONG\r\n")
    }

    #[test]
    fn it_should_echo() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("ECHO".to_string()),
                RedisDeserializationTypes::BulkString("Hello World".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+Hello World\r\n")
    }

    #[test]
    fn it_should_error_echo() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("ECHO".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "-Invalid Command\r\n")
    }

    #[test]
    fn it_should_error() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("123".to_string()),
                RedisDeserializationTypes::BulkString("Hello World".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "-Invalid Command\r\n")
    }

    #[test]
    fn it_should_set_and_get() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("set".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
                RedisDeserializationTypes::BulkString("Felipe".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+OK\r\n");
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("get".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+Felipe\r\n");
    }

    #[test]
    fn it_should_set_and_get_overwrite() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("set".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
                RedisDeserializationTypes::BulkString("Felipe".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+OK\r\n");
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("set".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
                RedisDeserializationTypes::BulkString("Carlos".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+OK\r\n");
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("get".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+Carlos\r\n");
    }

    #[test]
    fn it_should_fail_set() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("set".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "-Invalid Command\r\n");
    }

    #[test]
    fn it_should_fail_get() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("get".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "-Invalid Command\r\n");
    }

    #[test]
    fn it_should_get_none() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("get".to_string()),
                RedisDeserializationTypes::BulkString("Age".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+NONE\r\n");
    }
}
