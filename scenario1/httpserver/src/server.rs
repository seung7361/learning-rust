use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Server {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");

            let mut buffer = [0; 90];
            stream.read(&mut buffer).unwrap();

            // println!("{:#?}", String::from_utf8(buffer.to_vec()).unwrap());
            // "GET / HTTP/1.1\r\nHost: localhost:3000\r\nConnection: keep-alive\r\nsec-ch-ua: \"Google Chrome\";v"
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}