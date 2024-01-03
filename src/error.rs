#[cfg(unix)]
use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid character in name")]
    InvalidCharacter,
    #[error("Inappropriate count for sempaphore")]
    InappropriateCount,
    #[cfg(unix)]
    #[error("Failed to create named semaphore: {0}")]
    CreateFailed(#[source] io::Error),
    #[cfg(windows)]
    #[error("Failed to create named semaphore: {0}")]
    CreateFailed(#[source] windows::core::Error),
    #[cfg(unix)]
    #[error("Failed to wait semaphore: {0}")]
    WaitFailed(#[source] io::Error),
    #[cfg(windows)]
    #[error("Failed to wait semaphore: {0}")]
    WaitFailed(#[source] windows::core::Error),
    #[error("Named semaphore would block")]
    WouldBlock,
    #[cfg(unix)]
    #[error("Failed to wait semaphore: {0}")]
    PostFailed(#[source] io::Error),
    #[cfg(windows)]
    #[error("Failed to wait semaphore: {0}")]
    PostFailed(#[source] windows::core::Error),
    #[error("Unexpected error")]
    Unexpected,
}
