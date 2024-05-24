use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use super::deserialize::deserialize;

pub fn handle_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        let mut buffer = [0; 1024];

        if let Ok(size) = stream.read(&mut buffer) {
            let command = String::from_utf8_lossy(&buffer[..size]);
            let mut command = command.as_ref();

            let result = deserialize(&mut command).unwrap();

            println!("{:?}", result);

            let response = "+PONG\r\n".to_string();
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    });
}
