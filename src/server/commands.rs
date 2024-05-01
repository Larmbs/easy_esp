use std::net::SocketAddr;

/// Defines commands that can be sent to the server.
#[derive(Clone, Debug)]
pub enum ServerCMD {
    /// Instructs the server to shut down with the specified error code.
    ShutDown(u32), // Error code

    /// Instructs the server to send a message to all connected clients.
    SendAll(String), // Message

    /// Instructs the server to kick a certain client identified by its socket address.
    Kick(SocketAddr), // Kick a certain client
}