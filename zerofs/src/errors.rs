use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a file system operation.
pub type FsResult<T> = Result<T, FsError>;

/// An error that occurred during a file system operation.
#[derive(Debug, Error)]
pub enum FsError {}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `FsResult` d.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> FsResult<T> {
    Result::Ok(value)
}
