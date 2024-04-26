use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::sync::broadcast::Receiver;

use super::handler::Handler;

pub struct Conn<H>
where
    H: Handler + Sync + Send,
{
    stream: TcpStream,
    handler: Arc<Mutex<H>>,
    rx: Receiver<String>,
}

impl<H> Conn<H>
where
    H: Handler + Sync + Send,
{
    /// Creates a new conn instance
    pub fn new(stream: TcpStream, handler: Arc<Mutex<H>>, rx: Receiver<String>) -> Self {
        Conn {
            stream,
            handler,
            rx,
        }
    }

    /// Listens for peer and uses handler function to respond
    pub async fn listen(&mut self) {
        loop {
            // Waiting for readable socket
            //self.stream.readable().await.unwrap();

            if let Ok(message) = self.rx.try_recv() {
                println!("{}", message);
                self.send_message(&message).await.unwrap();
            }

            let mut buf = [0; 4096];

            match self.stream.try_read(&mut buf) {
                Ok(0) => {
                    continue;
                }
                Ok(n) => {
                    // n > 0
                    let data = &buf[..n];
                    let data_string = String::from_utf8_lossy(data).to_string();

                    let response = self.handler.lock().unwrap().handle_request(data_string);
                    let _ = self.send_message(&response).await;
                }
                Err(e) => {
                    //println!("{:?}", e);
                    continue;
                }
            }
        }
    }

    /// Listen for send requests
    pub async fn listen_for_server_request(&mut self) {
        while let Ok(message) = self.rx.recv().await {
            self.send_message(&message).await.unwrap();
        }
    }

    /// Sends a message to peer
    pub async fn send_message(&self, message: &String) -> Result<(), Box<dyn Error>> {
        loop {
            // Wait for the socket to be writable
            self.stream.writable().await?;

            match self.stream.try_write(message.as_bytes()) {
                Ok(n) => {
                    println!("[Server] sent {} bytes!", n);
                    break;
                }

                Err(e) => {
                    println!("{:?}", e);
                    return Err(Box::new(e));
                }
            }
        }

        Ok(())
    }
}
