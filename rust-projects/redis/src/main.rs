use redis::modules::{server::handle_connection, store::Redis};
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let redis = Redis::new();
    let redis = Arc::new(Mutex::new(redis));

    for wrapped_stream in listener.incoming() {
        let stream = wrapped_stream.unwrap();
        handle_connection(stream, Arc::clone(&redis));
    }
}
