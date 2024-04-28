//! This module defines types and traits for handling requests and responses in a server.

use std::net::SocketAddr;
use super::ServerCMD;

pub type Request = String;    /// Represents a request received by the server.
pub type Response = String;   /// Represents a response to be sent by the server.

/// Trait for handling incoming requests.
pub trait RequestHandler {
    /// Handles an incoming request and returns a response along with an optional server command.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming request.
    /// * `origin` - The socket address of the client originating the request.
    ///
    /// # Returns
    ///
    /// A tuple containing the response to the request and an optional server command.
    /// 
    fn handle_request(&self, request: Request, origin: SocketAddr) -> (Response, Option<ServerCMD>);
}



/// Example implementation for handling requests in a chat room.
pub struct ChatRoomHandler {}
#[allow(dead_code, unused)]
impl ChatRoomHandler {
    pub fn new() -> Self {
        ChatRoomHandler {}
    }
}
#[allow(dead_code, unused)]
impl RequestHandler for ChatRoomHandler {
    fn handle_request(&self, request: Request, origin: SocketAddr) -> (Response, Option<ServerCMD>) {
        // Handle the request, e.g., in a chat room scenario
        let response = format!("Ok");

        // Optionally, send a command to the server, e.g., broadcasting the message to all clients
        let cmd = ServerCMD::SendAll(format!("{}: {}", origin, request));
        (response, Some(cmd))
    }
}

/// Test implementation for handler that just send request back to client
pub struct SendBackHandler {}
#[allow(dead_code, unused)]
impl SendBackHandler {
    pub fn new() -> Self {
        SendBackHandler {}
    }
}
#[allow(dead_code, unused)]
impl RequestHandler for SendBackHandler {
    fn handle_request(&self, request: Request, origin: SocketAddr) -> (Response, Option<ServerCMD>) {
        (request, None)
    }
}
