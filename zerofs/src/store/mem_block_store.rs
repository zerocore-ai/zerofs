use std::{collections::HashMap, future::Future, sync::Arc};

use bytes::Bytes;
use tokio::sync::RwLock;

use crate::{BlockId, BlockStore, FsError, FsResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A block store that stores blocks in memory.
pub struct MemBlockStore {
    blocks: Arc<RwLock<HashMap<BlockId, Bytes>>>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl BlockStore for MemBlockStore {
    fn read_block(&self, block_id: BlockId) -> impl Future<Output = FsResult<Bytes>> {
        let blocks = self.blocks.clone();
        async move {
            let blocks = blocks.read().await;
            match blocks.get(&block_id) {
                Some(data) => Ok(data.clone()),
                None => Err(FsError::BlockNotFound { block_id }),
            }
        }
    }

    fn write_block(
        &self,
        block_id: BlockId,
        data: impl Into<Bytes>,
    ) -> impl Future<Output = FsResult<()>> {
        let blocks = self.blocks.clone();
        async move {
            let mut blocks = blocks.write().await;
            blocks.insert(block_id, data.into());
            Ok(())
        }
    }

    fn delete_block(&self, block_id: BlockId) -> impl Future<Output = FsResult<()>> {
        let blocks = self.blocks.clone();
        async move {
            let mut blocks = blocks.write().await;
            if blocks.remove(&block_id).is_none() {
                return Err(FsError::BlockNotFound { block_id });
            }
            Ok(())
        }
    }
}

impl Default for MemBlockStore {
    fn default() -> Self {
        MemBlockStore {
            blocks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mem_block_store() {
        let store = MemBlockStore::default();

        let block_id = BlockId::default();
        let data = Bytes::from("hello, world!");

        store.write_block(block_id, data.clone()).await.unwrap();

        let read_data = store.read_block(block_id).await.unwrap();
        assert_eq!(data, read_data);

        store.delete_block(block_id).await.unwrap();

        let result = store.read_block(block_id).await;
        assert_eq!(result, Err(FsError::BlockNotFound { block_id }));
    }
}
