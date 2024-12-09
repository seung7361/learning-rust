use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("localhost:3000").unwrap();
    println!("Running on port 3000");

    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}