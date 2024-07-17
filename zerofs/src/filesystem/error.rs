use std::{error::Error, fmt::Display};

use thiserror::Error;

use super::{DescriptorFlags, OpenFlags, Path};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a file system operation.
pub type FsResult<T> = Result<T, FsError>;

/// An error that occurred during a file system operation.
#[derive(Debug, Error)]
pub enum FsError {
    /// Infallible error.
    #[error("Infallible error")]
    Infallible(#[from] core::convert::Infallible),

    /// When a path segment is invalid.
    #[error("Invalid path segment: {0:?}")]
    InvalidPathSegment(String),

    /// Not a file.
    #[error("Not a file: {0:?}")]
    NotAFile(Option<Path>),

    /// Not a directory.
    #[error("Not a directory: {0:?}")]
    NotADirectory(Option<Path>),

    /// Not a file or directory.
    #[error("Not a file or directory: {0:?}")]
    NotAFileOrDir(Option<Path>),

    /// Not found.
    #[error("Not found: {0}")]
    NotFound(Path),

    /// Leading `.` in path.
    #[error("Leading `.` in path")]
    LeadingCurrentDir,

    /// Out of bounds `..` in path.
    #[error("Out of bounds `..` in path")]
    OutOfBoundsParentDir,

    /// UCAN error.
    #[error("UCAN error: {0}")]
    Ucan(#[from] zeroutils_ucan::UcanError),

    /// Custom error.
    #[error("Custom error: {0}")]
    Custom(#[from] AnyError),

    /// DID related error.
    #[error("DID error: {0}")]
    Did(#[from] zeroutils_did_wk::DidError),

    /// IPLD Store error.
    #[error("IPLD Store error: {0}")]
    IpldStore(#[from] zeroutils_store::StoreError),

    /// Invalid deserialized OpenFlag value
    #[error("Invalid OpenFlag value: {0}")]
    InvalidOpenFlag(u8),

    /// Invalid deserialized EntityFlag value
    #[error("Invalid EntityFlag value: {0}")]
    InvalidEntityFlag(u8),

    /// Invalid deserialized PathFlag value
    #[error("Invalid PathFlag value: {0}")]
    InvalidPathFlag(u8),

    /// Permission error.
    #[error("Permission error: {0}")]
    PermissionError(#[from] PermissionError),

    /// Wrong file descriptor flags.
    #[error("Wrong file descriptor flags: path: {0}, descriptor_flags: {1:?}")]
    WrongFileDescriptorFlags(Path, DescriptorFlags),

    /// Need at least READ flag set on the descriptor flags.
    #[error(
        "Need at least READ flag set on the descriptor flags: path: {0}, descriptor_flags: {1:?}"
    )]
    NeedAtLeastReadFlag(Path, DescriptorFlags),

    /// Open flags has EXCLUSIVE but entity already exists.
    #[error("Open flags has EXCLUSIVE but entity already exists: path: {0}, open_flags: {1:?}")]
    OpenFlagsExclusiveButEntityExists(Path, OpenFlags),

    /// Open flags has DIRECTORY but entity not a directory.
    #[error("Open flags has DIRECTORY but entity not a directory: path: {0}, open_flags: {1:?}")]
    OpenFlagsDirectoryButEntityNotADir(Path, OpenFlags),

    /// Invalid open flags combination.
    #[error("Invalid open flags combination: path: {0}, open_flags: {1:?}")]
    InvalidOpenFlagsCombination(Path, OpenFlags),

    /// Symlink not supported yet.
    #[error("Symlink not supported yet: path: {0}")]
    SymLinkNotSupportedYet(Path),
}

/// Permission error.
#[derive(Debug, Error)]
pub enum PermissionError {
    /// Child descriptor has higher permission than parent.
    #[error("Child descriptor has higher permission than parent: path: {0}, parent(descriptor_flags: {1:?}) child (descriptor_flags: {2:?}, open_flags: {3:?})")]
    ChildPermissionEscalation(Path, DescriptorFlags, DescriptorFlags, OpenFlags),
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
