use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::sync::broadcast::{Receiver, Sender};

use super::handler::Handler;
use super::ServerCMD;
use super::errors::ConnectionError;


pub struct Conn<H>
where
    H: Handler + Sync + Send,
{
    stream: TcpStream,
    handler: Arc<Mutex<H>>,
    rx: Receiver<String>,
    tx: Sender<ServerCMD>,
}

impl<H> Conn<H>
where
    H: Handler + Sync + Send,
{
    /// Creates a new conn instance
    pub fn new(stream: TcpStream, handler: Arc<Mutex<H>>, rx: Receiver<String>, tx: Sender<ServerCMD>) -> Self {
        Conn {
            stream,
            handler,
            rx,
            tx,
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }

    /// Listens for peer and uses handler function to respond
    pub async fn listen(&mut self) {
        loop {
            // Tries to listen for any new server requests
            if let Ok(message) = self.rx.try_recv() {
                println!("{}", message);
                self.send_message(&message).await.unwrap();
            }

            let mut buf = [0; 4096];

            // Waiting for read and dealing with it
            match self.stream.try_read(&mut buf) {
                Ok(0) => {
                    continue;
                }
                Ok(n) => {
                    // n > 0
                    println!("[Server] received message from {} of length {}", self.get_addr(), n);
                    let data = &buf[..n];
                    let data_string = String::from_utf8_lossy(data).to_string();

                    let (response, cmd) = self.handler.lock().unwrap().handle_request(data_string);
                    if let Some(cmd) = cmd {
                        self.tx.send(cmd).unwrap();
                    }
                    let _ = self.send_message(&response).await;
                }
               Err(_) => {
                    continue;
                }
            }
        }
    }

    /// Sends a message to peer
    pub async fn send_message(&self, message: &String) -> Result<(), ConnectionError> {
        loop {
            // Wait for the socket to be writable
            self.stream.writable().await.map_err(|_| {ConnectionError::TimedOut})?;

            match self.stream.try_write(message.as_bytes()) {
                Ok(n) => {println!("[Server] sent {} bytes!", n); break;}
                Err(e) => {eprintln!("{:?}", e); return Err(ConnectionError::TimedOut);}
            }
        }

        Ok(())
    }
}
