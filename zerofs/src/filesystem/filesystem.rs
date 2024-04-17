use ucan::Ucan;

use crate::BlockStore;

use super::{FileSystemBuilder, FsPath};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A file system with capability-based security.
#[derive(Debug)]
pub struct FileSystem<S>
where
    S: BlockStore,
{
    /// The store used to persist blocks in the file system.
    pub(crate) _block_store: S,

    /// The root capability of the file system.
    pub(crate) _root_cap: Ucan,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> FileSystem<S>
where
    S: BlockStore,
{
    /// Creates a file system builder.
    pub fn builder() -> FileSystemBuilder {
        FileSystemBuilder::default()
    }

    /// Opens a file at the given path.
    pub fn open_at(&self, _path: impl Into<FsPath>) {
        unimplemented!()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use anyhow::Ok;

    use crate::{fixture, MemBlockStore};

    use super::*;

    #[tokio::test]
    async fn test_file_system() -> anyhow::Result<()> {
        let mem_store = MemBlockStore::default();

        let _fs = FileSystem::<MemBlockStore>::builder()
            .block_store(mem_store)
            .root_cap(Ucan::try_from(fixture::root_cap().await)?)
            .build();

        // let dir = fs.open_at();
        // let file = fs.create_dir_at(path);

        Ok(())
    }
}
