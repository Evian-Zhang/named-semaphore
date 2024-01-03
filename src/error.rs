#[cfg(unix)]
use std::io;

use thiserror::Error;

/// Common errors when dealing semaphore.
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid character in name
    #[error("Invalid character in name")]
    InvalidCharacter,
    /// Inappropriate count for semaphore
    #[error("Inappropriate count for sempaphore")]
    InappropriateCount,
    /// Failed to create named semaphore.
    #[cfg(unix)]
    #[error("Failed to create named semaphore: {0}")]
    CreateFailed(#[source] io::Error),
    /// Failed to create named semaphore.
    #[cfg(windows)]
    #[error("Failed to create named semaphore: {0}")]
    CreateFailed(#[source] windows::core::Error),
    /// Failed to wait semaphore
    #[cfg(unix)]
    #[error("Failed to wait semaphore: {0}")]
    WaitFailed(#[source] io::Error),
    /// Failed to wait semaphore
    #[cfg(windows)]
    #[error("Failed to wait semaphore: {0}")]
    WaitFailed(#[source] windows::core::Error),
    /// Named semaphore would block.
    ///
    /// This error will only occur when calling [`NamedSemaphore::try_wait`][crate::NamedSemaphore::try_wait]
    #[error("Named semaphore would block")]
    WouldBlock,
    /// Failed to post semaphore.
    #[cfg(unix)]
    #[error("Failed to post semaphore: {0}")]
    PostFailed(#[source] io::Error),
    /// Failed to post semaphore.
    #[cfg(windows)]
    #[error("Failed to post semaphore: {0}")]
    PostFailed(#[source] windows::core::Error),
    /// Unexpected error.
    #[error("Unexpected error")]
    Unexpected,
}
