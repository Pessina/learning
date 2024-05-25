use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use super::{commands::execute_command, deserialize::deserialize};

pub fn handle_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        let mut buffer = [0; 1024];

        if let Ok(size) = stream.read(&mut buffer) {
            let command = String::from_utf8(Vec::from(&buffer[..size])).unwrap();
            let command = deserialize(&mut command.as_ref()).unwrap();

            let response = execute_command(&command);

            println!("Response {response}");

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    });
}
