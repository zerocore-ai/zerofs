use std::convert::TryInto;

use zeroutils_store::{IpldStore, PlaceholderStore};
use zeroutils_ucan::SignedUcan;

use super::{Entity, EntityFlags, EntityType, FsResult, Metadata, OpenFlags, Path, PathFlags};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a directory in the `zerofs` file system.
///
/// Since zerofs is a capability-based file system, a `UCAN` needs to provided that lets the file
/// system grant access to the directory's contents.
#[derive(Debug)]
pub struct Dir<S>
where
    S: IpldStore,
{
    /// The name of the directory.
    _name: String,

    /// Directory metadata.
    _metadata: Metadata,

    /// The store used to persist blocks in the directory.
    _store: S,

    /// The entries in the directory.
    _entries: Vec<Entity<S>>,
}

/// A builder for constructing a zerof directory or file system.
pub struct DirBuilder<S = ()> {
    store: S,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Dir<PlaceholderStore> {
    /// Creates a file system builder.
    pub fn builder() -> DirBuilder {
        DirBuilder::default()
    }
}

impl<S> Dir<S>
where
    S: IpldStore,
{
    /// Opens the file, directory at the given path.
    pub fn open_at(
        &self,
        _path: impl TryInto<Path>,
        _path_flags: PathFlags,
        _open_flags: OpenFlags,
        _entity_flags: EntityFlags,
        _ucan: SignedUcan<S>,
    ) -> FsResult<Entity<S>> {
        unimplemented!()
    }
}

impl<S> DirBuilder<S> {
    /// Sets the block store for the file system.
    pub fn store<T>(self, store: T) -> DirBuilder<T>
    where
        T: IpldStore,
    {
        DirBuilder { store }
    }
}

impl<S> DirBuilder<S>
where
    S: IpldStore,
{
    /// Builds the file system.
    pub fn build(self) -> Dir<S> {
        Dir {
            _metadata: Metadata::new(EntityType::Dir),
            _name: "/".to_string(),
            _store: self.store,
            _entries: Vec::new(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for DirBuilder {
    fn default() -> Self {
        DirBuilder { store: () }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use zeroutils_key::{Ed25519KeyPair, KeyPairGenerate};
    use zeroutils_store::MemoryStore;

    use super::*;

    #[tokio::test]
    async fn test_fs_open_at() -> anyhow::Result<()> {
        let mem_store = MemoryStore::default();
        let _keypair = Ed25519KeyPair::generate(&mut rand::thread_rng())?;

        let _fs = Dir::builder().store(mem_store).build();

        // let file = fs.open_at("cats/tabby.txt")?.as_file().unwrap();

        // let _read_stream = file.read(10)?;
        // let dir = fs.create_dir_at(path);

        Ok(())
    }
}
