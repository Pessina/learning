use redis::modules::server::handle_connection;
use std::{io::Read, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for wrapped_stream in listener.incoming() {
        let mut stream = wrapped_stream.unwrap();
        handle_connection(stream);
    }
}
