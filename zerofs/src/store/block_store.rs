use std::future::Future;

use bytes::Bytes;
use cid::Cid;

use crate::FsResult;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A unique identifier for a block of data.
pub type BlockId = Cid;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// `BlockStore` is an asynchronous key-value store that maps block IDs to blocks of data.
pub trait BlockStore {
    /// Read a block of data from the store.
    fn read_block(&self, block_id: BlockId) -> impl Future<Output = FsResult<Bytes>>;

    /// Write a block of data to the store.
    fn write_block(
        &self,
        block_id: BlockId,
        data: impl Into<Bytes>,
    ) -> impl Future<Output = FsResult<()>>;

    /// Delete a block of data from the store.
    fn delete_block(&self, block_id: BlockId) -> impl Future<Output = FsResult<()>>;
}
