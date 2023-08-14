use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::{fs, env};

fn main() {
    let listener = TcpListener::bind("127.1:7878").unwrap();
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}