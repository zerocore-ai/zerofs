use std::{error::Error, fmt::Display};

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a file system operation.
pub type FsResult<T> = Result<T, FsError>;

/// An error that occurred during a file system operation.
#[derive(Debug, Error)]
pub enum FsError {
    /// When a path segment is invalid.
    #[error("Invalid path segment: {0:?}")]
    InvalidPathSegment(String),

    /// Not a file.
    #[error("Not a file")]
    NotAFile,

    /// Not a directory.
    #[error("Not a directory")]
    NotADirectory,

    /// UCAN error.
    #[error("UCAN error: {0}")]
    Ucan(#[from] zeroutils_ucan::UcanError),

    /// Custom error.
    #[error("Custom error: {0}")]
    Custom(#[from] AnyError),

    /// DID related error.
    #[error("DID error: {0}")]
    Did(#[from] zeroutils_did_wk::DidError),
}

/// An error that can represent any error.
#[derive(Debug)]
pub struct AnyError {
    error: anyhow::Error,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl FsError {
    /// Creates a new `Err` result.
    pub fn custom(error: impl Into<anyhow::Error>) -> FsError {
        FsError::Custom(AnyError {
            error: error.into(),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `FsResult`.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> FsResult<T> {
    Result::Ok(value)
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl PartialEq for AnyError {
    fn eq(&self, other: &Self) -> bool {
        self.error.to_string() == other.error.to_string()
    }
}

impl Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for AnyError {}
