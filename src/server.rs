use super::handler::Handler;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{self, Sender, Receiver};
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
    cmd_rx: Receiver<ServerCMD>,
    cmd_tx: Sender<ServerCMD>,
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
            self.cmd_tx.clone(),
        );
        
        let handle = tokio::spawn(async move {
            conn.listen().await;
        });
        
        self.handles.push(handle);
    }

    /// Send all
    pub fn send_all(&self, message: String) {
        self.send_all_tx.send(message).unwrap();
    }
    
    /// Start listening for incoming connections
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
            };
        }
    }
    
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

#[derive(Clone, Debug)]
pub enum ServerCMD {
    ShutDown(u32),      // Error code
    SendAll(String),    // Message
    Kick(SocketAddr),   // Kick a certain client
}
