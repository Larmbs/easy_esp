//! This module defines a simple TCP server that implements the observer pattern.
//!
//! The server manages client connections and validates message formats. It also
//! facilitates sending and receiving messages between clients.
//!
//! # Example
//!
//! ```
//! use std::net::SocketAddr;
//! use easy_esp::{Server, SendBackHandler};
//!
//! #[tokio::main]
//! async fn main() {
//!     let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//!     let server = Server::new(addr, SendBackHandler::new());
//!
//!     server.listen().await;
//! }
//! ```

use super::handler::RequestHandler;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::task::JoinHandle;

mod conn;
use conn::Conn;

mod commands;
pub use commands::ServerCMD;

/// The server manages client connections, verifies message formats, and handles sending and receiving messages.
///
/// # Type Parameters
///
/// * `H`: Type that implements the `RequestHandler` trait for handling incoming requests.
///
/// # Fields
///
/// * `address`: The socket address on which the server is listening.
/// * `handles`: Vector of join handles for spawned connection tasks.
/// * `send_all_tx`: Sender channel for broadcasting messages to all clients.
/// * `cmd_rx`: Receiver channel for receiving commands from clients.
/// * `cmd_tx`: Sender channel for sending commands to clients.
/// * `message_handler`: Shared handler function to handle all incoming messages.
pub struct Server<H>
where
    H: RequestHandler + Sync + Send + 'static,
{
    address: SocketAddr,

    handles: Vec<JoinHandle<()>>,
    send_all_tx: Sender<String>,
    cmd_rx: Receiver<ServerCMD>,
    cmd_tx: Sender<ServerCMD>,
    message_handler: Arc<Mutex<H>>, // Shared handler func to handle all incoming messages
}

impl<H> Server<H>
where
    H: RequestHandler + Sync + Send,
{
    /// Gets the socket address of the server.
    ///
    /// # Returns
    ///
    /// The socket address on which the server is listening.
    pub fn get_addr(&self) -> SocketAddr {
        self.address
    }

    /// Adds a new connection to the server.
    ///
    /// # Arguments
    ///
    /// * `conn_stream` - The TCP stream representing the connection to the client.
    pub fn add_conn(&mut self, conn_stream: TcpStream) {
        let mut conn = Conn::new(
            conn_stream,
            self.message_handler.clone(),
            self.send_all_tx.subscribe(),
            self.cmd_tx.clone(),
        );

        let handle = tokio::spawn(async move {
            conn.listen().await;
        });

        self.handles.push(handle);
    }

    /// Sends a message to all connected clients.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be sent to all clients.
    pub fn send_all(&self, message: String) {
        self.send_all_tx.send(message).unwrap();
    }

    /// Starts listening for incoming connections and commands.
    pub async fn listen(&mut self) {
        println!("[Server] starting on {}...", self.get_addr());
        let listener = TcpListener::bind(self.address).await.unwrap();

        loop {
            tokio::select! {
                // Accept a new connection
                Ok((socket, addr)) = listener.accept() => {
                    println!("[Server] Received a new connection from {}", addr);
                    self.add_conn(socket);
                },
                // Receive a message from the rx channel
                Ok(cmd) = self.cmd_rx.recv() => {
                    match cmd {
                        ServerCMD::ShutDown(code) =>  {
                            println!("[Server] Server shutting down with code {}...", code);
                        }
                        ServerCMD::SendAll(message) => {
                            self.send_all(message);
                        }
                        ServerCMD::Kick(addr) => {
                            println!("[Server] Kicking client with addr {}", addr);
                        }
                    }
                }
            }
        }
    }

    /// Creates a new instance of `Server`.
    ///
    /// # Arguments
    ///
    /// * `address` - The socket address on which the server will listen for incoming connections.
    /// * `message_handler` - The handler for processing incoming requests.
    ///
    /// # Returns
    ///
    /// A new instance of `Server`.
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
