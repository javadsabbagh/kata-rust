use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Cannot bind to address 0.0.0.0:8080");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Error in accepting client connection: {:?}", e),
            Ok(stream) => {
                thread::spawn(move || {
                  handle_client(stream).unwrap_or_else(|err| eprintln!("There was error: {:?}", err))
                });
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("New client from address: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()); }
        stream.write(&buf[..bytes_read])?;
    }
}
