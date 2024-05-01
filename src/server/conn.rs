use std::{
    io,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
// Tokio modules
use tokio::net::TcpStream;
use tokio::sync::broadcast::{Receiver, Sender};

use super::{ConnCMD, ServerCMD};
use crate::error::ConnectionError;
use crate::handler::RequestHandler;
use crate::message::{convert_to_json, parse_message_from_bytes, Message};
/// Represents a connection to a client.
pub struct Conn<H>
where
    H: RequestHandler + Sync + Send,
{
    stream: TcpStream,
    handler: Arc<Mutex<H>>,
    rx: Receiver<ConnCMD>,
    tx: Sender<ServerCMD>,
}

impl<H> Conn<H>
where
    H: RequestHandler + Sync + Send,
{
    /// Creates a new connection instance.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream representing the connection to the client.
    /// * `handler` - The request handler for processing incoming requests.
    /// * `rx` - The receiver channel for receiving messages from the server.
    /// * `tx` - The sender channel for sending commands to the server.
    ///
    /// # Returns
    ///
    /// A new `Conn` instance.
    pub fn new(
        stream: TcpStream,
        handler: Arc<Mutex<H>>,
        rx: Receiver<ConnCMD>,
        tx: Sender<ServerCMD>,
    ) -> Self {
        // Telling handler about new client
        let cmd = handler
            .lock()
            .unwrap()
            .client_connect(stream.peer_addr().unwrap());

        // Executing handlers request
        if let Some(cmd) = cmd {
            tx.send(cmd).unwrap();
        }
        Conn {
            stream,
            handler,
            rx,
            tx,
        }
    }

    /// Gets the socket address of the client.
    ///
    /// # Returns
    ///
    /// The socket address of the client.
    pub fn get_addr(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }

    /// Listens for messages from the client and responds using the handler function.
    pub async fn listen(&mut self) {
        loop {
            // Tries to listen for any new server requests
            if let Ok(cmd) = self.rx.try_recv() {
                match cmd {
                    ConnCMD::Send(message) => {
                        self.send_message(message).await.unwrap();
                    }
                    ConnCMD::Kick => {
                        break;
                    }
                }
            }

            let mut buf = [0; 4096];

            // Waiting for read and dealing with it
            match self.stream.try_read(&mut buf) {
                Ok(0) => {
                    println!("Disconnected");
                    break;
                }
                Ok(n) => {
                    let data = &buf[..n];

                    let message = if let Ok(message) = parse_message_from_bytes(data) {
                        message
                    } else {
                        println!("Could not parse message");
                        continue;
                    };

                    println!("[Server <- {}] received ({:?}))!", self.get_addr(), message);

                    let (response, cmd) = self
                        .handler
                        .lock()
                        .unwrap()
                        .handle_request(message, self.get_addr());

                    if let Some(cmd) = cmd {
                        self.tx.send(cmd).unwrap();
                    }
                    let _ = self.send_message(response).await;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::ConnectionReset {
                    } else {
                        eprintln!("Failed to read from socket: {}", e);
                    }
                    break;
                }
            }
        }
    }

    /// Sends a message to the client.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be sent to the client.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the message was successfully sent or an error occurred.
    pub async fn send_message(&self, message: Message) -> Result<(), ConnectionError> {
        let message_json = convert_to_json(message);
        loop {
            // Wait for the socket to be writable
            self.stream
                .writable()
                .await
                .map_err(|_| ConnectionError::TimedOut)?;

            match self.stream.try_write(message_json.as_bytes()) {
                Ok(_) => {
                    println!("[Server -> {}] sent ({})!", self.get_addr(), message_json);
                    break;
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Err(ConnectionError::TimedOut);
                }
            }
        }

        Ok(())
    }
}

impl<H> Drop for Conn<H>
where
    H: RequestHandler + Sync + Send,
{
    fn drop(&mut self) {
        // Making sure handler knows of client disconnect
        let cmd = self
            .handler
            .lock()
            .unwrap()
            .client_disconnect(self.get_addr());

        // If the handler wants to make a server request then ok
        if let Some(cmd) = cmd {
            self.tx.send(cmd).unwrap();
        }

        println!("[Server] Disconnected with {}", self.get_addr());
    }
}
