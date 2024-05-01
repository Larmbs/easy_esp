use std::net::SocketAddr;
use crate::message::Message;

/// Defines commands that can be sent to the server.
#[derive(Clone, Debug)]
pub enum ServerCMD {
    /// Instructs the server to shut down with the specified error code.
    ShutDown(u32), // Error code

    /// Instructs the server to send a message to all connected clients.
    SendAll(Message), // Message

    /// Instructs the server to kick a certain client identified by its socket address.
    Kick(SocketAddr), // Kick a certain client
}

/// Set of commands the server can send a conn object
#[derive(Clone, Debug)]
pub enum ConnCMD {
    /// Stops socket connection
    Kick,

    /// Send some message to your client
    Send(Message), 
}