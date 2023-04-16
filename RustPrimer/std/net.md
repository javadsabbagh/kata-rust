# Network module: Echo of W cat

In this example, W Mao will lead you to write a TCP ECHO software that everyone has written but no one has used, as the end of this chapter. This program is just an example program, and I personally estimate that no one uses her in real life. Still, it suffices as an example of the standard library.

First, we need a server side.

```rust
fn server<A: ToSocketAddrs>(addr: A) -> io::Result<()> {
     // Create a listener
     let listener = try!(TcpListener::bind(&addr)) ;
     // This program only needs to process one link at a time
     for stream in listener.incoming() {
         // unpack the stream again by match
         match stream {
             // The focus of matching here is how to pass a mut match to a Result
             Ok(mut st) => {
                 // We always ask the client to send data first
                 // Prepare a very large buffer
                 // Of course, in real life we generally use ring buffers to reuse memory.
                 // Here is only for demonstration, it is a very inefficient way
                 let mut buf: Vec<u8> = vec![0u8; 1024];
                 // unpack by try! method
                 // The point of the try! method is that it needs to have a specific Error type to cooperate with it
                 let rcount = try!(st. read(&mut buf));
                 // Only output the content read in the buffer
                 println!("{:?}", &buf[0..rcount]);
                 // write back content
                 let wcount = try!(st.write(&buf[0..rcount]));
                 // The following code is actually logic processing
                 // not part of the standard library anymore
                 if rcount != wcount {
                     panic!("Not Fully Echo!, r={}, w={}", rcount, wcount);
                 }
                 // Clear what has been read
                 buf. clear();
             }
             Err(e) => {
                 panic!("{}", e);
             }
         }
     }
     // Close the connection on the server side
     drop(listener);
     Ok(())
}

```


Then, we prepare a client that simulates a TCP short link:

```rust
fn client<A: ToSocketAddrs>(addr: A) -> io::Result<()> {

     let mut buf = vec![0u8;1024];
     loop {
         // Compared with Listener, TcpStream is much simpler
         // This simulation is the process of tcp short link, which can be regarded as a basic IO simulation of a typical HTTP interaction
         // Of course, there is no HTTP protocol in this newsletter XD!
         let mut stream = TcpStream::connect(&addr).unwrap();
         let msg = "WaySLOG comming!".as_bytes();
         // Avoid sending data too fast and swiping the screen
         thread::sleep_ms(100);
         let rcount = try!(stream.write(&msg));
         let _ = try!(stream. read(&mut buf));
         println!("{:?}", &buf[0..rcount]);
         buf. clear();
     }
     Ok(())
}

```

Stitching our program together looks like this:

```rust
use std::net::*;
use std::io;
use std::io::{Read, Write};
use std::env;
use std::thread;

fn server<A: ToSocketAddrs>(addr: A) -> io::Result<()> { .. }


fn client<A: ToSocketAddrs>(addr: A) -> io::Result<()> { .. }


fn main() {
    let mut args = env::args();
    args.next();
    let action = args.next().unwrap();
    if action == "s" {
        server(&args.next().unwrap()).unwrap();
    } else {
        client(&args.next().unwrap()).unwrap();
    }
}

```

You can try the result yourself


Writing network programs is destined to deal with all kinds of magical conditions and errors, defining your own data structure, sticky packets, etc., all need us to deal with and pay attention to. In comparison, Rust's own network infrastructure construction is not satisfactory, and even network I/O only provides the above block I/O. Perhaps its team is more concerned with the improvement of the language's basic grammatical features and compilation, but in fact, it is very important to have such an official network library. At the same time, I also hope that more network library solutions will emerge from Rust to make Rust's future better and brighter.
