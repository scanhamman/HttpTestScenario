use std::io::{Read, Write};  // Bring Read abnnd Write traits into scope
use std::net::TcpListener;

// Implementers of the Read trait are required to implement one method: read(). 
// This allows us to use the same read() method to read from a File, Stdin, 
// TcpStream, or any other type that implements the Read trait.
// Implementers of the Write trait implement two methods: write () and flush (). 
// Examples of types that implement the Write trait include File, Stderr, Stdout, 
// and TcpStream.

fn main() {
    // initialise socket server
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    
    // socket server waits for incoming connections
    for stream in connection_listener.incoming() {
        
        // when a new connection is requested it is of type Result<TcpStream, Error>.
        // When unwrapped this returns a TcpStream if successful - otherwise it panics.
        let  mut stream = stream.unwrap(); // stream made mutable so read/write possible
        println!("Connection established!");
        let mut buffer = [0;1024];  
        stream.read(&mut buffer).unwrap();  // Read fro the incoming stream
        stream.write(&mut buffer).unwrap();  // Echo back whatever is received to the client on the same connection
       }
}
