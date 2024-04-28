use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Connection Time Out")] 
    TimedOut,
}
