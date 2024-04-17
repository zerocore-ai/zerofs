use ucan::Ucan;

use crate::{BlockStore, MemBlockStore};

use super::FileSystem;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A builder for constructing a zerofs file system.
pub struct FileSystemBuilder<S = MemBlockStore, C = ()>
where
    S: BlockStore,
{
    block_store: S,
    root_cap: C,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S, C> FileSystemBuilder<S, C>
where
    S: BlockStore,
{
    /// Sets the block store for the file system.
    pub fn block_store<T>(self, block_store: T) -> FileSystemBuilder<T, C>
    where
        T: BlockStore,
    {
        FileSystemBuilder {
            block_store,
            root_cap: self.root_cap,
        }
    }

    /// Sets the root capability for the file system.
    pub fn root_cap(self, root_cap: Ucan) -> FileSystemBuilder<S, Ucan> {
        FileSystemBuilder {
            block_store: self.block_store,
            root_cap,
        }
    }
}

impl<S> FileSystemBuilder<S, Ucan>
where
    S: BlockStore,
{
    /// Builds the file system.
    pub fn build(self) -> FileSystem<S> {
        FileSystem {
            _block_store: self.block_store,
            _root_cap: self.root_cap,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for FileSystemBuilder {
    fn default() -> Self {
        FileSystemBuilder {
            block_store: MemBlockStore::default(),
            root_cap: (),
        }
    }
}
