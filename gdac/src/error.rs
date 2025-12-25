//! Error types for gdac

use thiserror::Error;

/// Result type alias for gdac operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur in gdac operations
#[derive(Error, Debug)]
pub enum Error {
    /// Lua execution or interaction error
    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Generic error wrapper
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
