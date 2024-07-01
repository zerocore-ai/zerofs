use core::fmt;
use std::fmt::Debug;

use zeroutils_store::{ipld::cid::Cid, IpldStore, Storable, StoreResult};

use super::{
    DescriptorFlags, Dir, DirHandle, File, FileHandle, FsError, FsResult, Handle, Metadata, Symlink,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// This is an entity in the file system.
#[derive(Clone)]
pub enum Entity<S>
where
    S: IpldStore,
{
    /// A file.
    File(File<S>),

    /// A directory.
    Dir(Dir<S>),

    /// A symlink.
    Symlink(Symlink<S>),
}

/// A handle for an open file system entity.
#[derive(Debug)]
pub struct EntityHandle<S>(Handle<Entity<S>>)
where
    S: IpldStore + Send + Sync;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> Entity<S>
where
    S: IpldStore + Send + Sync,
{
    /// Returns true if the entity is a file.
    pub fn is_file(&self) -> bool {
        matches!(self, Entity::File(_))
    }

    /// Returns true if the entity is a directory.
    pub fn is_dir(&self) -> bool {
        matches!(self, Entity::Dir(_))
    }

    /// Tries to convert the entity to a file.
    pub fn as_file(self) -> FsResult<File<S>> {
        if let Entity::File(file) = self {
            return Ok(file);
        }

        Err(FsError::NotAFile(None))
    }

    /// Tries to convert the entity to a directory.
    pub fn as_dir(self) -> FsResult<Dir<S>> {
        if let Entity::Dir(dir) = self {
            return Ok(dir);
        }

        Err(FsError::NotADirectory(None))
    }

    /// Returns the metadata for the directory.
    pub fn metadata(&self) -> &Metadata {
        match self {
            Entity::File(file) => file.metadata(),
            Entity::Dir(dir) => dir.metadata(),
            Entity::Symlink(symlink) => symlink.metadata(),
        }
    }

    /// Change the store used to persist the entity.
    pub fn use_store<T>(self, store: T) -> Entity<T>
    where
        T: IpldStore,
    {
        match self {
            Entity::File(file) => Entity::File(file.use_store(store)),
            Entity::Dir(dir) => Entity::Dir(dir.use_store(store)),
            Entity::Symlink(symlink) => Entity::Symlink(symlink.use_store(store)),
        }
    }
}

impl<S> EntityHandle<S>
where
    S: IpldStore + Send + Sync,
{
    /// Returns the flags for the entity.
    pub fn flags(&self) -> &DescriptorFlags {
        self.0.flags()
    }

    /// Creates a new descriptor from an entity.
    pub fn from_entity(entity: Entity<S>, flags: DescriptorFlags) -> Self {
        EntityHandle(Handle::new(entity, flags))
    }

    /// Creates a new descriptor for a file.
    pub fn from_file(file: File<S>, flags: DescriptorFlags) -> Self {
        EntityHandle(Handle::new(Entity::File(file), flags))
    }

    /// Creates a new descriptor for a directory.
    pub fn from_dir(dir: Dir<S>, flags: DescriptorFlags) -> Self {
        EntityHandle(Handle::new(Entity::Dir(dir), flags))
    }

    /// Tries to convert the descriptor to a file descriptor.
    pub fn as_file(self) -> FsResult<FileHandle<S>> {
        let flags = *self.0.flags();
        self.0
            .entity
            .as_file()
            .map(|file| FileHandle::new(file, flags))
    }

    /// Tries to convert the descriptor to a directory descriptor.
    pub fn as_dir(self) -> FsResult<DirHandle<S>> {
        let flags = *self.0.flags();
        self.0.entity.as_dir().map(|dir| DirHandle::new(dir, flags))
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S> Storable<S> for Entity<S>
where
    S: IpldStore + Send + Sync,
{
    async fn store(&self) -> StoreResult<Cid> {
        match self {
            Entity::File(file) => file.store().await,
            Entity::Dir(dir) => dir.store().await,
            Entity::Symlink(symlink) => symlink.store().await,
        }
    }

    async fn load(_cid: &Cid, _store: S) -> StoreResult<Self> {
        // TODO: Implement
        unimplemented!()
    }
}

impl<S> Debug for Entity<S>
where
    S: IpldStore + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entity::File(file) => f.debug_tuple("File").field(file).finish(),
            Entity::Dir(dir) => f.debug_tuple("Dir").field(dir).finish(),
            Entity::Symlink(symlink) => f.debug_tuple("Symlink").field(symlink).finish(),
        }
    }
}
