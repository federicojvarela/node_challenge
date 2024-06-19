use std::io;
use thiserror::Error;
use tokio::time::error::Elapsed;

#[derive(Debug, Error)]
pub enum ConectionError {
    #[error("Connection failed: {0:?}")]
    ConnectionFailed(io::Error),

    #[error("Connection timed out")]
    ConnectionTimedOut(Elapsed),

    #[error("Connection lost")]
    ConnectionLost,

    #[error("Sending failed")]
    SendingFailed(io::Error),
}
