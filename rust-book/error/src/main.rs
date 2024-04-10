use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(err) => panic!("Problem creating the file: {:?}", err),
            },
            other_error => panic!("File doesn't exist: {:?}", other_error),
        },
    };

    let file2 = File::open("hello.txt").unwrap();
    let file3 = File::open("hello.txt").expect("File doesn't exist");

    let username = read_username_from_file("username.txt");

    match username {
        Ok(name) => println!("{name}"),
        Err(e) => panic!("{e}"),
    }
}

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut username: String = String::new();

    File::open(file_name)?.read_to_string(&mut username)?;

    Ok(username)
}
