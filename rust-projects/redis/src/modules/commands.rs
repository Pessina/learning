use std::{num::NonZeroU8, result, sync::{Arc, Mutex}};

use chrono::{Duration, TimeZone, Utc};

use super::{
    store::{Redis, RedisCell}, string_array::ArrayPlacement, types::RedisDeserializationTypes
};

const INVALID_COMMAND: &'static str = "-Invalid Command\r\n";
const OK_COMMAND: &'static str = "+OK\r\n";

/// Performs an arithmetic operation on a value stored in Redis at a given key.
///
/// This function locks the Redis store, retrieves the value associated with the specified key,
/// applies a user-defined arithmetic function `f` to it, and stores the result back into Redis.
/// If the key does not exist, and a default value is provided, it sets the key to this default value.
///
/// # Arguments
/// * `redis` - A shared, mutable reference to the Redis store wrapped in an Arc and Mutex.
/// * `key` - The key in the Redis store where the value is stored.
/// * `f` - A closure that defines the arithmetic operation to perform on the retrieved value.
/// * `default` - An optional default value to use if the key does not exist in Redis.
///
/// # Returns
/// * `Ok(())` if the operation was successful.
/// * `Err(())` if the operation failed, including if the value could not be parsed as an integer.
pub fn arithmetic_command<F>(redis: &Arc<Mutex<Redis>>, key: &str, f: F, default: Option<i64>) -> Result<(), ()>
where
    F: Fn(i64) -> i64 {
        let mut redis = redis.lock().unwrap();
        let value = match redis.get(key) {
            Some(value) => {
                value.value.parse::<i64>().map(|number| RedisCell {
                    value: (f(number)).to_string(),
                    expiry: value.expiry
                }).ok()
            }
            None => default.map(|default| RedisCell {
                value: default.to_string(), 
                expiry: None
            })
        };

        match value {
            Some(value) => {
                redis.set(key.to_string(), value); 
                Ok(())
            },
            None => Err(())
        }
}

/// Executes a given Redis command by deserializing it and applying the corresponding operation on the Redis store.
///
/// # Arguments
/// * `command` - A reference to the deserialized Redis command to be executed.
/// * `redis` - An `Arc<Mutex<Redis>>` shared among threads, allowing synchronized access to the Redis store.
///
/// # Returns
/// A `String` representing the result of the command execution, which could be a success message, error message, or data retrieved from the store.
pub fn execute_command(command: &RedisDeserializationTypes, redis: Arc<Mutex<Redis>>) -> String {
    let ret = match command {
        RedisDeserializationTypes::Array(a) => match a.as_slice() {
            [RedisDeserializationTypes::BulkString(c), args @ ..] => match c.as_ref() {
                "PING" => Some("+PONG\r\n".to_string()),
                "ECHO" => match args {
                    [RedisDeserializationTypes::BulkString(echo)] => {
                        Some(format!("+{}\r\n", echo))
                    }
                    _ => None,
                },
                "SET" => match args {
                    [RedisDeserializationTypes::BulkString(key), RedisDeserializationTypes::BulkString(value), rest @ ..] =>
                    {
                        let expiry = match rest {
                            [] => None,
                            [RedisDeserializationTypes::BulkString(expiry_config), RedisDeserializationTypes::BulkString(expiry_value)] => {
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

                        Some(OK_COMMAND.to_string().to_string())
                    }
                    _ => None,
                },
                "GET" => match args {
                    [RedisDeserializationTypes::BulkString(key)] => {
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
                    .filter_map(|x| {
                        if let RedisDeserializationTypes::BulkString(k) = x {
                            Some(k)
                        } else {
                            None
                        }
                    })
                    .fold(0, |acc, k| {
                        match redis.lock().unwrap().get(k) {
                            None => acc,
                            Some(_) => acc + 1
                        }
                    });

                    Some(format!("+{}\r\n", count))
                }
                "DEL" => {
                    let count = args
                        .iter()
                        .filter_map(|x| {
                            if let RedisDeserializationTypes::BulkString(s) = x {
                                Some(s)
                            } else {
                                None
                            }
                        })
                        .fold(0, |acc, x| {
                            match redis.lock().unwrap().delete(x) {
                                Some(_) => acc + 1,
                                None => acc,
                            }
                        });

                    Some(format!("+{}\r\n", count))
                }, 
                command @ "INCR" | command @ "DECR" => {
                    match args {
                        [RedisDeserializationTypes::BulkString(key)] => {
                            let operation = if command == "INCR" { |x| x + 1 } else { |x| x - 1 };
                            match arithmetic_command(&redis, key, operation, Some(0)) {
                                Ok(_) => Some(OK_COMMAND.to_string()),
                                Err(_) => Some("-Invalid operation on string\r\n".to_string())
                            }
                        }
                        _ => None
                    }
                }
                placement @ "LPUSH" | placement @ "RPUSH" => {
                    match args {
                        [RedisDeserializationTypes::BulkString(key), arr_elements @ ..]
                            if arr_elements.iter().all(|e| matches!(e, RedisDeserializationTypes::BulkString(_))) =>
                        {
                            let result = arr_elements.iter().try_fold(0, |_, e| {
                                if let RedisDeserializationTypes::BulkString(value) = e {
                                    redis.lock().unwrap().set_list(key.clone(), value.clone(), if placement == "LPUSH" { ArrayPlacement::LEFT } else { ArrayPlacement::RIGHT } )
                                } else {
                                    Err(INVALID_COMMAND.to_string())
                                }
                            });

                            match result {
                                Ok(len) => Some(format!("+{}\r\n", len)),
                                Err(err) => Some(format!("-{}\r\n", err)),
                            }
                        }
                        _ => None,
                    }
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

    use std::{thread, time::Duration};
    
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

    struct SetExpiryArgs {
        config: String,
        value: String,
    }

    fn build_command(args: Vec<RedisDeserializationTypes>) -> RedisDeserializationTypes {
        RedisDeserializationTypes::Array(Box::new(args))
    }

    fn execute_set(
        redis: Arc<Mutex<Redis>>,
        key: String,
        value: String,
        expiry: Option<SetExpiryArgs>,
    ) -> String {
        let mut command = vec![
            RedisDeserializationTypes::BulkString("SET".to_string()),
            RedisDeserializationTypes::BulkString(key),
            RedisDeserializationTypes::BulkString(value),
        ];

        match expiry {
            Some(args) => {
                command.extend([
                    RedisDeserializationTypes::BulkString(args.config),
                    RedisDeserializationTypes::BulkString(args.value),
                ]);
            }
            None => {}
        };

        execute_command(&build_command(command), redis)
    }

    fn execute_get(redis: Arc<Mutex<Redis>>, key: String) -> String {
        execute_command(
            &build_command(vec![
                RedisDeserializationTypes::BulkString("GET".to_string()),
                RedisDeserializationTypes::BulkString(key),
            ]),
            redis,
        )
    }

    fn execute_exist(redis: Arc<Mutex<Redis>>, keys: Vec<String>) -> String {
        let mut command = vec![RedisDeserializationTypes::BulkString("EXIST".to_string())];
        command.extend(
            keys.iter()
                .map(|x| RedisDeserializationTypes::BulkString(x.to_string())),
        );

        execute_command(&build_command(command), redis)
    }

    fn execute_del(redis: Arc<Mutex<Redis>>, keys: Vec<String>) -> String {
        let mut command = vec![RedisDeserializationTypes::BulkString("DEL".to_string())];
        command.extend(
            keys.iter()
                .map(|x| RedisDeserializationTypes::BulkString(x.to_string())),
        );

        execute_command(&build_command(command), redis)
    }

    fn execute_array_push(redis: Arc<Mutex<Redis>>, key: String, values: Vec<String>, placement: ArrayPlacement) -> String {
        let mut command = vec![RedisDeserializationTypes::BulkString((if placement == ArrayPlacement::LEFT { "LPUSH" } else { "RPUSH" }).to_string())];

        command.extend([
            RedisDeserializationTypes::BulkString(key),
        ]);

        command.extend(values.iter().map(|v| RedisDeserializationTypes::BulkString(v.to_string())));

        execute_command(&build_command(command), redis)
    }

    #[derive(PartialEq)]
    enum ArithmeticCommand {
        INCR, 
        DECR
    }

    fn execute_incr_or_decr(redis: Arc<Mutex<Redis>>, key: String, command: ArithmeticCommand) -> String {
        execute_command(&build_command(vec![
            RedisDeserializationTypes::BulkString((if command == ArithmeticCommand::INCR { "INCR" } else { "DECR" }).to_string()),
            RedisDeserializationTypes::BulkString(key),
        ]), redis)
    }

    #[test]
    fn it_should_ping_pong() {
        let Setup { redis } = setup();
        let response = execute_command(
            &build_command(vec![RedisDeserializationTypes::BulkString(
                "PING".to_string(),
            )]),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+PONG\r\n")
    }

    #[test]
    fn it_should_echo() {
        let Setup { redis } = setup();
        let response = execute_command(
            &build_command(vec![
                RedisDeserializationTypes::BulkString("ECHO".to_string()),
                RedisDeserializationTypes::BulkString("Hello World".to_string()),
            ]),
            Arc::clone(&redis),
        );
        assert_eq!(response, "+Hello World\r\n")
    }

    #[test]
    fn it_should_error_echo() {
        let Setup { redis } = setup();
        let response = execute_command(
            &build_command(vec![RedisDeserializationTypes::BulkString(
                "ECHO".to_string(),
            )]),
            Arc::clone(&redis),
        );
        assert_eq!(response, "-Invalid Command\r\n")
    }

    #[test]
    fn it_should_error() {
        let Setup { redis } = setup();
        let response = execute_command(
            &build_command(vec![
                RedisDeserializationTypes::BulkString("123".to_string()),
                RedisDeserializationTypes::BulkString("Hello World".to_string()),
            ]),
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
            None,
        );

        assert_eq!(response, OK_COMMAND.to_string());
        let response = execute_get(Arc::clone(&redis), "Name".to_string());

        assert_eq!(response, "+Felipe\r\n");
    }

    #[test]
    fn it_should_set_and_get_overwrite() {
        let Setup { redis } = setup();
        let response = execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            None,
        );
        assert_eq!(response, OK_COMMAND.to_string());

        let response = execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Carlos".to_string(),
            None,
        );
        assert_eq!(response, OK_COMMAND.to_string());

        let response = execute_get(redis, "Name".to_string());

        assert_eq!(response, "+Carlos\r\n");
    }

    #[test]
    fn it_should_fail_set() {
        let Setup { redis } = setup();
        let response = execute_command(
            &build_command(vec![
                RedisDeserializationTypes::BulkString("SET".to_string()),
                RedisDeserializationTypes::BulkString("Name".to_string()),
            ]),
            Arc::clone(&redis),
        );
        assert_eq!(response, "-Invalid Command\r\n");
    }

    #[test]
    fn it_should_fail_get() {
        let Setup { redis } = setup();
        let response = execute_command(
            &build_command(vec![RedisDeserializationTypes::BulkString(
                "GET".to_string(),
            )]),
            Arc::clone(&redis),
        );
        assert_eq!(response, "-Invalid Command\r\n");
    }

    #[test]
    fn it_should_get_none() {
        let Setup { redis } = setup();

        let response = execute_get(redis, "Age".to_string());
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_seconds() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            Some(SetExpiryArgs {
                config: "EX".to_string(),
                value: "1".to_string(),
            }),
        );

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_secs(2));

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_miliseconds() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            Some(SetExpiryArgs {
                config: "PX".to_string(),
                value: "1000".to_string(),
            }),
        );

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_millis(2000));

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_utc_seconds() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            Some(SetExpiryArgs {
                config: "EAXT".to_string(),
                value: (Utc::now() + ChronoDuration::seconds(1))
                    .timestamp()
                    .to_string(),
            }),
        );

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_secs(2));

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_be_expired_utc_miliseconds() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            Some(SetExpiryArgs {
                config: "PXAT".to_string(),
                value: ((Utc::now() + ChronoDuration::milliseconds(1000)).timestamp() * 1000)
                    .to_string(),
            }),
        );

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+Felipe\r\n");

        thread::sleep(Duration::from_millis(2000));

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    fn it_should_exist_1() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            None,
        );

        let response = execute_exist(Arc::clone(&redis), vec!["Name".to_string()]);

        assert_eq!(response, "+1\r\n");
    }

    #[test]
    fn it_should_exist_3() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            None,
        );

        execute_set(
            Arc::clone(&redis),
            "Age".to_string(),
            "23".to_string(),
            None,
        );

        execute_set(
            Arc::clone(&redis),
            "Country".to_string(),
            "Portugal".to_string(),
            None,
        );

        let response = execute_exist(
            Arc::clone(&redis),
            vec!["Name".to_string(), "Age".to_string(), "Country".to_string()],
        );

        assert_eq!(response, "+3\r\n");
    }

    #[test]
    fn it_should_not_exist() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            None,
        );

        execute_set(
            Arc::clone(&redis),
            "Age".to_string(),
            "23".to_string(),
            None,
        );

        execute_set(
            Arc::clone(&redis),
            "Country".to_string(),
            "Portugal".to_string(),
            None,
        );

        let response = execute_exist(
            Arc::clone(&redis),
            vec![
                "Sex".to_string(),
                "Language".to_string(),
                "Marital Status".to_string(),
            ],
        );

        assert_eq!(response, "+0\r\n")
    }

    #[test]
    pub fn should_delete() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            None,
        );

        let response = execute_del(Arc::clone(&redis), vec!["Name".to_string()]);
        assert_eq!(response, "+1\r\n");

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    pub fn should_delete_3() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            None,
        );
        execute_set(
            Arc::clone(&redis),
            "Age".to_string(),
            "23".to_string(),
            None,
        );
        execute_set(
            Arc::clone(&redis),
            "Country".to_string(),
            "UAE".to_string(),
            None,
        );

        let response = execute_del(
            Arc::clone(&redis),
            vec!["Name".to_string(), "Age".to_string(), "Country".to_string()],
        );
        assert_eq!(response, "+3\r\n");

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+NONE\r\n");
        let response = execute_get(Arc::clone(&redis), "Age".to_string());
        assert_eq!(response, "+NONE\r\n");
        let response = execute_get(Arc::clone(&redis), "Country".to_string());
        assert_eq!(response, "+NONE\r\n");
    }

    #[test]
    pub fn should_delete_0() {
        let Setup { redis } = setup();

        execute_set(
            Arc::clone(&redis),
            "Name".to_string(),
            "Felipe".to_string(),
            None,
        );
        execute_set(
            Arc::clone(&redis),
            "Age".to_string(),
            "23".to_string(),
            None,
        );
        execute_set(
            Arc::clone(&redis),
            "Country".to_string(),
            "UAE".to_string(),
            None,
        );

        let response = execute_del(
            Arc::clone(&redis),
            vec![
                "Sex".to_string(),
                "Marital Status".to_string(),
                "Income".to_string(),
            ],
        );
        assert_eq!(response, "+0\r\n");

        let response = execute_get(Arc::clone(&redis), "Name".to_string());
        assert_eq!(response, "+Felipe\r\n");
        let response = execute_get(Arc::clone(&redis), "Age".to_string());
        assert_eq!(response, "+23\r\n");
        let response = execute_get(Arc::clone(&redis), "Country".to_string());
        assert_eq!(response, "+UAE\r\n");
    }

    #[test]
    fn should_increment_set_0() {
        let Setup {redis} = setup(); 

        let response = execute_incr_or_decr(Arc::clone(&redis), "New".to_string(), ArithmeticCommand::INCR);
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_get(Arc::clone(&redis), "New".to_string());
        assert_eq!(response, "+0\r\n".to_string());
    }

    #[test]
    fn should_increment() {
        let Setup {redis} = setup(); 

        let response = execute_incr_or_decr(Arc::clone(&redis), "New".to_string(), ArithmeticCommand::INCR);
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_incr_or_decr(Arc::clone(&redis), "New".to_string(), ArithmeticCommand::INCR);
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_get(Arc::clone(&redis), "New".to_string());
        assert_eq!(response, "+1\r\n".to_string());
    }

    #[test]
    fn should_increment_set() {
        let Setup {redis} = setup(); 

        let response = execute_set(Arc::clone(&redis), "New".to_string(), "12".to_string(), None);
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_incr_or_decr(Arc::clone(&redis), "New".to_string(), ArithmeticCommand::INCR);
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_get(Arc::clone(&redis), "New".to_string());
        assert_eq!(response, "+13\r\n".to_string());
    }

    #[test]
    fn should_fail_increment() {
        let Setup {redis} = setup(); 

        let response = execute_set(Arc::clone(&redis), "New".to_string(), "not number".to_string(), None);
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_incr_or_decr(Arc::clone(&redis), "New".to_string(), ArithmeticCommand::INCR);
        assert_eq!(response, "-Invalid operation on string\r\n".to_string());

        let response = execute_get(Arc::clone(&redis), "New".to_string());
        assert_eq!(response, "+not number\r\n".to_string());
    }

    #[test]
    fn should_not_change_expiry_increment_on_valid_number() {
        let Setup {redis} = setup(); 

        let expiry_time = (Utc::now() + ChronoDuration::seconds(600)).timestamp();

        let response = execute_set(Arc::clone(&redis), "New".to_string(), "10".to_string(),
         Some(SetExpiryArgs {
                    config: "EAXT".to_string(),
                    value: expiry_time.to_string(),
        }));
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_incr_or_decr(Arc::clone(&redis), "New".to_string(), ArithmeticCommand::INCR);
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_get(Arc::clone(&redis), "New".to_string());
        assert_eq!(response, "+11\r\n".to_string());

        if let Some(value) = redis.lock().unwrap().get("New") {
            assert_eq!(value.expiry.unwrap(), Utc.timestamp_opt(expiry_time, 0).unwrap());
            assert_eq!(value.value, "11".to_string());
        };
    }

    #[test]
    fn should_not_change_expiry_increment_on_invalid_number() {
        let Setup {redis} = setup(); 

        let expiry_time = (Utc::now() + ChronoDuration::seconds(600)).timestamp();

        let response = execute_set(Arc::clone(&redis), "New".to_string(), "not number".to_string(),
         Some(SetExpiryArgs {
                    config: "EAXT".to_string(),
                    value: expiry_time.to_string(),
        }));
        assert_eq!(response, OK_COMMAND.to_string().to_string());

        let response = execute_incr_or_decr(Arc::clone(&redis), "New".to_string(), ArithmeticCommand::INCR);
        assert_eq!(response, "-Invalid operation on string\r\n".to_string());

        let response = execute_get(Arc::clone(&redis), "New".to_string());
        assert_eq!(response, "+not number\r\n".to_string());

        if let Some(value) = redis.lock().unwrap().get("New") {
            assert_eq!(value.expiry.unwrap(), Utc.timestamp_opt(expiry_time, 0).unwrap());
            assert_eq!(value.value, "not number".to_string());
        };
    }

    #[test]
    fn should_create_and_insert_on_array_lpush() {
        let Setup { redis } = setup();

        let response = execute_array_push(Arc::clone(&redis), "array".to_string(), vec![
            "element1".to_string(),
            "element2".to_string(),
            "element3".to_string(),
            "element4".to_string(),
        ], ArrayPlacement::LEFT);
        assert_eq!("+4\r\n".to_string(), response);

        let response = execute_get(redis, "array".to_string());
        assert_eq!("+[element4,element3,element2,element1]\r\n".to_string(), response);
    }

    #[test]
    fn should_fail_insert_on_array() {
        let Setup { redis } = setup();

        execute_set(Arc::clone(&redis), "not_array".to_string(), "not_array".to_string(), None);

        let response = execute_array_push(Arc::clone(&redis), "not_array".to_string(), vec![
            "element1".to_string(),
        ], ArrayPlacement::LEFT);

        assert_eq!("-The string it's not an array\r\n".to_string(), response);
    }

    #[test]
    fn should_create_and_insert_on_array_rpush() {
        let Setup { redis } = setup();

        let response = execute_array_push(Arc::clone(&redis), "array".to_string(), vec![
            "element1".to_string(),
            "element2".to_string(),
            "element3".to_string(),
            "element4".to_string(),
        ], ArrayPlacement::RIGHT);
        assert_eq!("+4\r\n".to_string(), response);

        let response = execute_get(redis, "array".to_string());
        assert_eq!("+[element1,element2,element3,element4]\r\n".to_string(), response);
    }
}
