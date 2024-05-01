//! This module provides functionality for handling server operations, including request handling,
//! server management, error handling, and logging.

mod server;
mod errors;
mod logging;

pub use server::{Server, ServerCMD, RequestHandler};
