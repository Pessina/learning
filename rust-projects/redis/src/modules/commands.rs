use std::sync::{Arc, Mutex};

use chrono::{Duration, TimeZone, Utc};

use super::{
    store::{Redis, RedisCell},
    types::RedisDeserializationTypes,
};

const INVALID_COMMAND: &'static str = "-Invalid Command\r\n";

pub fn execute_command(command: &RedisDeserializationTypes, redis: Arc<Mutex<Redis>>) -> String {
    let ret = match command {
        RedisDeserializationTypes::Array(a) => match a.as_slice() {
            [RedisDeserializationTypes::BulkString(ref c), args @ ..] => match c.as_ref() {
                "PING" => Some("+PONG\r\n".to_string()),
                "ECHO" => match args {
                    [RedisDeserializationTypes::BulkString(ref echo)] => {
                        Some(format!("+{}\r\n", echo))
                    }
                    _ => None,
                },
                "SET" => match args {
                    [RedisDeserializationTypes::BulkString(ref key), RedisDeserializationTypes::BulkString(ref value), rest @ ..] =>
                    {
                        let expiry = match rest {
                            [] => None,
                            [RedisDeserializationTypes::BulkString(ref expiry_config), RedisDeserializationTypes::BulkString(ref expiry_value)] => {
                                Some(match expiry_config.as_ref() {
                                    "EX" => {
                                        Utc::now()
                                            + Duration::seconds(expiry_value.parse().unwrap())
                                    }
                                    "PX" => {
                                        Utc::now()
                                            + Duration::milliseconds(expiry_value.parse().unwrap())
                                    }
                                    "EAXT" => {
                                        Utc.timestamp_opt(expiry_value.parse().unwrap(), 0).unwrap()
                                    }
                                    "PXAT" => Utc
                                        .timestamp_opt(
                                            (expiry_value.parse::<i64>().unwrap()) / 1000,
                                            0,
                                        )
                                        .unwrap(),
                                    _ => return INVALID_COMMAND.to_string(),
                                })
                            }
                            _ => return INVALID_COMMAND.to_string(),
                        };

                        redis.lock().unwrap().set(
                            key.to_string(),
                            RedisCell {
                                value: value.to_string(),
                                expiry,
                            },
                        );

                        Some("+OK\r\n".to_string())
                    }
                    _ => None,
                },
                "GET" => match args {
                    [RedisDeserializationTypes::BulkString(ref key)] => {
                        match redis.lock().unwrap().get(key) {
                            Some(result) => Some(format!("+{}\r\n", result.value)),
                            None => Some(format!("+NONE\r\n")),
                        }
                    }
                    _ => None,
                },
                "EXIST" => {
                    let count = args
                    .iter()
                    .map(|x| if let RedisDeserializationTypes::BulkString(k) = x { k } else { panic!() })
                    .fold(0, |acc, x| {
                            match redis.lock().unwrap().get(x) {
                                None => acc, 
                                Some(_) => acc + 1
                            }
                    });
                
                    Some(format!("+{}\r\n", count))
                }
                // Mock config, to bypass redis-benchmark request
                "CONFIG" => {
                    Some("*2\r\n$4\r\nsave\r\n$23\r\n3600 1 300 100 60 10000\r\n*2\r\n$10\r\nappendonly\r\n$2\r\nno\r\n"
                        .to_string())
                }
                _ => None
            },
            _ => None
        },
        _ => None
    };

    match ret {
        None => INVALID_COMMAND.to_string(),
        Some(ret) => ret,
    }
}

#[cfg(test)]
mod tests {

    use std::{
        thread::{self},
        time::Duration,
    };

    use chrono::{Duration as ChronoDuration, Utc};

    use super::*;

    struct Setup {
        redis: Arc<Mutex<Redis>>,
    }

    fn setup() -> Setup {
        Setup {
            redis: Arc::new(Mutex::new(Redis::new())),
        }
    }

    fn execute_set(redis: Arc<Mutex<Redis>>, key: String, value: String) -> String {
        execute_command(&RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("SET".to_string()),
                RedisDeserializationTypes::BulkString(key),
                RedisDeserializationTypes::BulkString(value)
        ])), redis)
    }

    fn execute_get(redis: Arc<Mutex<Redis>>, key: String) -> String {
        execute_command(&RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString(key),
        ])), redis)
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

        let response = execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
        );

        assert_eq!(response, "+OK\r\n");
        let response = execute_get(
            Arc::clone(&redis),
            "Name".to_string(),
        );

        assert_eq!(response, "+Felipe\r\n");
    }

    #[test]
    fn it_should_set_and_get_overwrite() {
        let Setup { redis } = setup();
        let response = execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
        );
        assert_eq!(response, "+OK\r\n");

        let response = execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Carlos".to_string(),
        );
        assert_eq!(response, "+OK\r\n");

        let response = execute_get(redis, "Name".to_string());

        assert_eq!(response, "+Carlos\r\n");
    }

    #[test]
    fn it_should_fail_set() {
        let Setup { redis } = setup();
        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("SET".to_string()),
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
                RedisDeserializationTypes::BulkString("GET".to_string()),
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
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Age".to_string()),
            ])),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_seconds() {
        let Setup { redis } = setup();

        execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("SET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
                RedisDeserializationTypes::BulkString("Felipe".to_string()),
                RedisDeserializationTypes::BulkString("EX".to_string()),
                RedisDeserializationTypes::BulkString("1".to_string()),
            ])),
            Arc::clone(&redis),
        );

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_secs(2));

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_miliseconds() {
        let Setup { redis } = setup();

        execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("SET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
                RedisDeserializationTypes::BulkString("Felipe".to_string()),
                RedisDeserializationTypes::BulkString("PX".to_string()),
                RedisDeserializationTypes::BulkString("1000".to_string()),
            ])),
            Arc::clone(&redis),
        );

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_millis(2000));

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_utc_seconds() {
        let Setup { redis } = setup();

        execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("SET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
                RedisDeserializationTypes::BulkString("Felipe".to_string()),
                RedisDeserializationTypes::BulkString("EAXT".to_string()),
                RedisDeserializationTypes::BulkString(
                    (Utc::now() + ChronoDuration::seconds(1))
                        .timestamp()
                        .to_string(),
                ),
            ])),
            Arc::clone(&redis),
        );

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_secs(2));

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_utc_miliseconds() {
        let Setup { redis } = setup();

        execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("SET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
                RedisDeserializationTypes::BulkString("Felipe".to_string()),
                RedisDeserializationTypes::BulkString("PXAT".to_string()),
                RedisDeserializationTypes::BulkString(
                    ((Utc::now() + ChronoDuration::milliseconds(1000)).timestamp() * 1000)
                        .to_string(),
                ),
            ])),
            Arc::clone(&redis),
        );

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_millis(2000));

        let response = execute_command(
            &RedisDeserializationTypes::Array(Box::new(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ])),
            Arc::clone(&redis),
        );

        assert_eq!(response, "+NONE\r\n");
    }
}
