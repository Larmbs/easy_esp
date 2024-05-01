//! This module defines types and traits for handling requests and responses in a server.

use super::ServerCMD;
use std::net::SocketAddr;

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
    fn handle_request(
        &mut self,
        request: String,
        origin: SocketAddr,
    ) -> (String, Option<ServerCMD>);

    /// Notifies the handler when a client connects to the server.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address of the client that connected.
    ///
    /// # Returns
    ///
    /// An optional server command.
    fn client_connect(&mut self, addr: SocketAddr) -> Option<ServerCMD>;

    /// Notifies the handler when a client disconnects from the server.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address of the client that disconnected.
    ///
    /// # Returns
    ///
    /// An optional server command.
    fn client_disconnect(&mut self, addr: SocketAddr) -> Option<ServerCMD>;
}
