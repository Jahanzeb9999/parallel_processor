use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProcessError {
    ChannelError,
    JoinError
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::ChannelError => write!(f, "Channel communication error"),
            ProcessError::JoinError => write!(f, "Join error")
        }
    }
}

impl Error for ProcessError {}

impl From<tokio::task::JoinError> for ProcessError {
    fn from(_: tokio::task::JoinError) -> Self {
        ProcessError::JoinError
    }
}