//! This module defines a simple TCP server that implements the observer pattern.
//!
//! The server manages client connections and validates message formats. It also
//! facilitates sending and receiving messages between clients.

// Dependencies and Imports
use std::{ net::SocketAddr, sync::{ Arc, Mutex } };
use tokio::{
    net::{ TcpListener, TcpStream },
    sync::broadcast::{ self, Receiver, Sender },
    task::JoinHandle,
};

// Submodules
mod conn; // Connection handling
mod commands; // Command processing
mod handler; // Request handling

// Public Interface
use conn::Conn;
pub use handler::RequestHandler;
pub use commands::{ ConnCMD, ServerCMD };

/// The server manages client connections, verifies message formats, and handles sending and receiving messages.
///
/// # Type Parameters
///
/// * `H`: Type that implements the `RequestHandler` trait for handling incoming requests.
pub struct Server<H> where H: RequestHandler + Sync + Send + 'static {
    address: SocketAddr,

    handles: Vec<JoinHandle<()>>,
    send_all_tx: Sender<ConnCMD>,
    cmd_rx: Receiver<ServerCMD>,
    cmd_tx: Sender<ServerCMD>,
    message_handler: Arc<Mutex<H>>, // Shared handler func to handle all incoming messages
}

impl<H> Server<H> where H: RequestHandler + Sync + Send {
    /// Returns local socket addr
    pub fn get_addr(&self) -> SocketAddr {
        self.address
    }

    /// Creates a new socket connection and thread
    fn add_conn(&mut self, conn_stream: TcpStream) {
        let mut conn = Conn::new(
            conn_stream,
            self.message_handler.clone(),
            self.send_all_tx.subscribe(),
            self.cmd_tx.clone()
        );

        let handle = tokio::spawn(async move {
            conn.listen().await;
        });

        self.handles.push(handle);
    }

    /// Sends all clients a command
    pub fn send_all(&self, cmd: ConnCMD) {
        self.send_all_tx.send(cmd).unwrap();
    }

    /// Starts listening for incoming connections and commands.
    pub async fn listen(&mut self) {
        println!("[Server] starting on {}...", self.get_addr());
        let listener = TcpListener::bind(self.address).await.unwrap();

        loop {
            tokio::select! {
                // Accept a new connection
                Ok((socket, addr)) = listener.accept() => {
                    println!("[Server] Connected with {}", addr);
                    self.add_conn(socket);
                },
                // Receive a message from the rx channel
                Ok(cmd) = self.cmd_rx.recv() => {
                    match cmd {
                        ServerCMD::ShutDown(code) =>  {
                            println!("[Server] Server shutting down with code {}...", code);
                            self.send_all(ConnCMD::Kick);
                        }
                        ServerCMD::SendAll(message) => {
                            self.send_all(ConnCMD::Send(message));
                        }
                        ServerCMD::Kick(addr) => {
                            println!("[Server] Kicking client with addr {}", addr);
                        }
                    }
                }
            }
        }
    }

    /// Creates a new server object
    pub fn new(address: SocketAddr, message_handler: H) -> Self {
        let message_handler: Arc<Mutex<H>> = Arc::new(Mutex::new(message_handler));
        let handles = vec![];

        // doesn't really matter the count
        let count = 16;

        let (send_all_tx, _) = broadcast::channel(count);
        let (cmd_tx, cmd_rx) = broadcast::channel::<ServerCMD>(count);

        Server {
            address,
            handles,
            send_all_tx,
            cmd_rx,
            cmd_tx,
            message_handler,
        }
    }
}
