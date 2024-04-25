use serde::{Deserialize, Serialize};
use std::io::{Result, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};
use serde_json;

//mod tcp_conn;
use queued_rust::Queue;

#[cfg(test)]
mod tests;

///  F is a handler function
///  T is the data type being sent back and forth
///
pub struct TCPServer<F, T>
where
    F: Fn(T),
    T: Serialize + for<'a> Deserialize<'a>,
{
    listener: TcpListener,
    handler: F,
    send_queue: Queue<T>, // Represent a queue of internal requests to be sent to clients
}

impl<F, T> TCPServer<F, T>
where
    F: Fn(T),
    T: Serialize + for<'a> Deserialize<'a>,
{
    /// Starts TCP listener and server
    pub fn open(server_addr: SocketAddr, handler: F) -> Result<Self> {
        // Formatting add
        let addr = server_addr.to_string();
        let listener = TcpListener::bind(addr)?;
        let send_queue = Queue::new();
        Ok(Self { listener, handler, send_queue})
    }

    /// Get socket addr
    pub fn get_addr(&self) -> SocketAddr {
        self.listener.local_addr().unwrap()
    }

    /// Go through every send request and send it to the appropriate channel
    pub fn handle_send_requests(&mut self) {
        while let Some(item) = self.send_queue.first() {
            





        }
    }

    /// Handles request
    fn handle_request(&self, stream: &mut TcpStream) -> Result<()> {
        // Creating buffer reader
        let buf_reader = BufReader::new( stream);

        // Parse out buffer data into an object
        let data: T = serde_json::from_reader(buf_reader)?;
        
        // Calling handler func with data
        (self.handler)(data);

        Ok(())
    }

    /// Starts TCP server
    pub fn listen(&self) {
        // Listening for incoming TCP streams
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();

            if let Err(e) = self.handle_request(&mut stream) { // Handle request but if its error log it
                println!("{}", e);
            }
        }
    }
}
