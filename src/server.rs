use super::handler::Handler;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{self, Sender};
use tokio::task::JoinHandle;

use super::conn::Conn;
use tokio::net::{TcpListener, TcpStream};

/// Implements observer pattern
/// All this server does is manage clients and verify messaging format
/// as well as managing sending and receiving messages
pub struct Server<H>
where
    H: Handler + Sync + Send + 'static,
{
    address: SocketAddr,

    handles: Vec<JoinHandle<()>>,
    send_all_tx: Sender<String>,
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
    
    /// Add a conn
    pub fn add_conn(&mut self, conn_stream: TcpStream) {
        let mut conn = Conn::new(
            conn_stream,
            self.message_handler.clone(),
            self.send_all_tx.subscribe(),
        );
        
        let handle = tokio::spawn(async move {
            conn.listen().await;
        });
        
        self.handles.push(handle);
    }

    /// Send all
    pub fn send_all(&mut self, message: String) {
        self.send_all_tx.send(message).unwrap();
    }
    
    /// Start listening for incoming connections
    pub async fn listen(&mut self) {
        println!("[Server] starting on {}...", self.get_addr());
        let listener = TcpListener::bind(self.address).await.unwrap();

        loop {
            // The second item contains the IP and port of the new connection.
            if let Ok((socket, addr)) = listener.accept().await {
                println!("[Server] Received a new connection from {}", addr);
                self.add_conn(socket);
            }
        }
    }
    
    pub fn new(address: SocketAddr, message_handler: H) -> Self {
        let message_handler: Arc<Mutex<H>> = Arc::new(Mutex::new(message_handler));
        let handles = vec![];

        // doesn't really matter the count
        let count = 16;

        let (send_all_tx, _) = broadcast::channel(count);
        let (tx, rx) = broadcast::channel::<String>(count);
        
        Server {
            address,
            handles,
            send_all_tx,
            message_handler,
        }
    }
}
