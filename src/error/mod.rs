//! This module defines an error type for handling connection errors.

use thiserror::Error;

/// Represents an error that occurs during a connection operation.
#[derive(Error, Debug)]
pub enum ConnectionError {
    /// Indicates a connection timeout error.
    #[error("Connection Time Out")]
    TimedOut,
    
    /// Indicated that client aborted the connection early.
    #[error("Connection was aborted")]
    Aborted,

    /// Message sent to server was in unexpected format
    #[error("Unexpected protocol used")]
    ProtocolError,

    /// States that client cannot connect 
    #[error("Client is not authorized on network")]
    Unauthorized,
}
