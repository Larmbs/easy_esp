//! This module provides functionality for handling server operations, including request handling,
//! server management, error handling, and logging.

mod handler;
mod server;
mod errors;
mod logging;

pub use handler::{RequestHandler, ChatRoomHandler, SendBackHandler};
pub use server::{Server, ServerCMD};
