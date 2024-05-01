//! This module provides functionality for handling server operations, including request handling,
//! server management, error handling, and logging.

mod server;
mod error;
pub mod handler;
pub mod message;

pub use server::{Server, ServerCMD};
