use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid character in name")]
    InvalidCharacter,
    #[error("Failed to create named semaphore: {0}")]
    CreateFailed(#[source] io::Error),
    #[error("Failed to wait semaphore: {0}")]
    WaitFailed(#[source] io::Error),
    #[error("Named semaphore would block")]
    WouldBlock,
    #[error("Failed to wait semaphore: {0}")]
    PostFailed(#[source] io::Error),
}
