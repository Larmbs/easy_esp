use super::handler::Handler;
use std::io::{self, BufReader, Read, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use super::conn::Conn;
use tokio::net::{TcpListener, TcpStream};

/// Implements observer pattern
/// All this server does is manage clients and verify messaging format
/// as well as managing sending and receiving messages
pub struct Server<H>
where
    H: Handler + Sync + Send,
{
    address: SocketAddr,
    conns: Vec<Conn<H>>,

    handles: Vec<JoinHandle<()>>,

    message_handler: Arc<Mutex<H>>, // Shared handler func to handle all incoming messages
}

impl<H> Server<H>
where
    H: Handler + Sync + Send,
{
    /// Get socket addr
    pub fn get_addr(&self) -> SocketAddr {
        self.address
    }

    /// Drops all conns
    pub fn disconnect_all(&mut self) {
        self.conns.clear();
    }

    /// Add a conn
    // pub fn add_conn(&mut self, conn_stream: TcpStream) {
    //     let mut conn = Conn::new(conn_stream, self.message_handler.clone());
    //     self.conns.push(conn);

    //     let handle = thread::spawn(move || conn.listen());

    //     self.handles.push(handle);
    // }

    /// Send all
    pub fn send_all(&mut self, message: String) {
        for conn in &mut self.conns {
            conn.send_message(&message);
        }
    }

    /// Start listening for incoming connections
    pub async fn listen(&mut self) {
        println!("[Server] starting on {}...", self.get_addr());
        let listener = TcpListener::bind(self.address).await.unwrap();

        loop {
            // The second item contains the IP and port of the new connection.
            if let Ok((socket, addr)) = listener.accept().await {
                println!("[Server] Received a new connection from {}", addr);
            }
        }
    }

    pub async fn open(address: SocketAddr, message_handler: H) -> io::Result<Server<H>> {
        let conns = vec![];

        let message_handler = Arc::new(Mutex::new(message_handler));
        let handles = vec![];
        Ok(Server {
            address,
            conns,
            handles,
            message_handler,
        })
    }
}
