use std::future::Future;

use uuid::Uuid;

use crate::FsResult;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type BlockId = Uuid;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// `BlockStore` is an asynchronous key-value store that maps block IDs to blocks of data.
pub trait BlockStore {
    /// Read a block of data from the store.
    fn read_block(&self, block_id: BlockId) -> impl Future<Output = FsResult<Vec<u8>>>;

    /// Write a block of data to the store.
    fn write_block(&self, block_id: BlockId, data: &[u8]) -> impl Future<Output = FsResult<()>>;

    /// Delete a block of data from the store.
    fn delete_block(&self, block_id: BlockId) -> impl Future<Output = FsResult<()>>;
}
