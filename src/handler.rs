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
    fn handle_request(&mut self, request: Request, origin: SocketAddr) -> (Response, Option<ServerCMD>);

    fn client_connect(&mut self, addr: SocketAddr) -> Option<ServerCMD>;
    fn client_disconnect(&mut self, addr: SocketAddr) -> Option<ServerCMD>;
}
