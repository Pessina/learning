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

        // Keep the connection alive
        loop {
            match stream.read(&mut buffer) {
                // Close connection
                Ok(0) => {
                    return;
                }
                Ok(size) => {
                    if let Ok(command) = String::from_utf8(Vec::from(&buffer[..size])) {
                        if let Some(command) = deserialize(&mut command.as_ref()) {
                            let response = execute_command(&command, Arc::clone(&redis));

                            stream.write_all(response.as_bytes()).unwrap();
                            stream.flush().unwrap();
                        }
                    }
                }
                Err(_) => {
                    return;
                }
            }
        }
    });
}
