use std::net::{TcpStream, TcpListener, SocketAddr};
use std::io::{self, BufReader, Read, Write};
use super::handler::Handler;
use std::sync::{Mutex, Arc};

/// Implements observer pattern 
/// All this server does is manage clients and verify messaging format
/// as well as managing sending and receiving messages
struct Server<H> where H: Handler {
    verbose: bool,
    listener: TcpListener,
    clients: Vec<Client<H>>,

    message_handler: Arc<Mutex<H>>, // Shared handler func to handle all incoming messages
}

impl<H> Server<H> where H: Handler {
    /// Get socket addr
    pub fn get_addr(&self) -> SocketAddr {
        self.listener.local_addr().unwrap()
    }

    /// Drops all clients
    pub fn disconnect_all(&mut self) {
        self.clients.clear();
    }

    /// Add a client
    pub fn add_client(&mut self, client_stream: TcpStream) {
        let client = Client::new(client_stream, self.message_handler.clone());
        self.clients.push(client);
    }

    /// Send all
    pub fn send_all(&mut self, message: String) {
        for client in &mut self.clients {
            client.send_message(&message);
        }
    }

    /// Start listening for incoming connections
    pub fn listen(&self) {
        if self.verbose {println!("[Server] starting on {}...", self.get_addr())}

        // Listening for stream
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                println!("Received a new connection from {}", stream.peer_addr().unwrap());
            }
        }
    }

    pub async fn open(addr: SocketAddr, message_handler: H, verbose: bool) -> io::Result<Server<H>> {
        let listener = TcpListener::bind(addr)?;
        let clients = vec![];

        let message_handler = Arc::new(Mutex::new(message_handler));

        Ok(Server {
            verbose,
            listener,
            clients,
            message_handler
        })
    }
}

pub struct Client<H> where H: Handler {
    stream: TcpStream,
    handler: Arc<Mutex<H>>,
}

impl<H> Client<H> where H: Handler {
    /// Creates a new client instance
    pub fn new(stream: TcpStream, handler: Arc<Mutex<H>>) -> Self {
        Client {
            stream,
            handler,
        }
    }

    /// Listens for peer and uses handler function to respond
    pub fn listen(&mut self) {
        // Creating stream reader
        let mut buffer = [0; 1024]; 

        loop {
            // Checking for valid message
            if let Ok(bytes_len) = self.stream.read(&mut buffer) {
               if bytes_len > 0 {
                    // Reading data from buffer
                    let data = &buffer[..bytes_len];
                    let data_string = String::from_utf8_lossy(data).to_string();

                    // Calc response and writing to peer
                    let response = self.handler.lock().unwrap().handle_request(data_string);
                    self.send_message(&response);
               }
            }
        }
    }

    /// Sends a message to peer
    pub fn send_message(&mut self, message: &String) {
        self.stream.write(message.as_bytes()).unwrap();
    }
}

