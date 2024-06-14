use std::net::{Incoming, TcpListener};

fn main() {
    // initialise socket server
    let connection_listener = TcpListener::bind("127.0.0.:3000").unwrap();
    println!("Running on port 3000");
    
    // socket server waits for incoming connections
    for stream in connection_listener.incoming() {
        
        // when a new connection is requested it is of type Result<TcpStream, Error>.
        // When unwrapped this returns a TcpStream if successful - otherwise it panics.
        let _stream = stream.unwrap();
        println!("Connection established!");
    }
}
