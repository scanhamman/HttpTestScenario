use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;  // Utilities for the str primitive type

fn main() {
    // The TCP client initiates a connection to a remote server on localhost:3000
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();  // write message to the TCP connection
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();  // Read bytes received

    // Print out what is received from the server. The server sends raw bytes, 
    // and we have to convert them into UTF-8 str type to print them to the terminal.

    println!(
        "Got response fromn server:{:?}", str::from_utf8(&buffer).unwrap()
    );
}
