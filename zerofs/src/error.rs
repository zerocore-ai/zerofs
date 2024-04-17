use thiserror::Error;

use crate::BlockId;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a file system operation.
pub type FsResult<T> = Result<T, FsError>;

/// An error that occurred during a file system operation.
#[derive(Debug, Error, PartialEq)]
pub enum FsError {
    /// The block was not found.
    #[error("Block not found: {block_id}")]
    BlockNotFound {
        /// The ID of the block that was not found.
        block_id: BlockId,
    },
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `FsResult` d.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> FsResult<T> {
    Result::Ok(value)
}
