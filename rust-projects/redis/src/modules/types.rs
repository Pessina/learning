#[derive(Debug, PartialEq)]
pub enum RedisDeserializationTypes {
    SimpleString(String),
    ErrorMessage(String),
    Integer(i64),
    BulkString(String),
    Array(Box<Vec<RedisDeserializationTypes>>),
}
