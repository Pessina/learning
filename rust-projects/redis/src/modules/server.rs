use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use super::{commands::execute_command, deserialize::deserialize, store::Redis};

pub fn handle_connection(mut stream: TcpStream, redis: Arc<Mutex<Redis>>) {
    thread::spawn(move || {
        let mut buffer = [0; 1024];

        if let Ok(size) = stream.read(&mut buffer) {
            let command = String::from_utf8(Vec::from(&buffer[..size])).unwrap();
            let command = deserialize(&mut command.as_ref()).unwrap();

            println!("Command {:?}", command);

            let response = execute_command(&command, redis);

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    });
}
