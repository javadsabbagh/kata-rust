use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Cannot bind to address 0.0.0.0:8080");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Error in accepting client connection: {:?}", e),
            Ok(_) => {
                println!("Accepted new connection")
            }
        }
    }

}
