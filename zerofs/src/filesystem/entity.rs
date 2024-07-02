use core::fmt;
use std::{fmt::Debug, ops::Deref};

use zeroutils_store::{ipld::cid::Cid, IpldStore, Storable, StoreResult};

use super::{
    DescriptorFlags, Dir, DirHandle, File, FileHandle, FsError, FsResult, Handle, Metadata,
    PathSegment, RootDir, Symlink,
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
pub struct EntityHandle<S, T>(Handle<Entity<T>, S, T>)
where
    S: IpldStore,
    T: IpldStore;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> Entity<S>
where
    S: IpldStore,
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

impl<S, T> EntityHandle<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// Returns the flags for the entity.
    pub fn flags(&self) -> &DescriptorFlags {
        self.0.flags()
    }

    /// Creates a new handle from an entity, its name, descriptor flags, root directory, and path.
    ///
    /// ## Arguments
    ///
    /// * `entity` - The entity being referenced by the handle.
    /// * `name` - The name of the entity in its parent directory entries. `None` if the handle has
    ///   no parent directory.
    /// * `flags` - The descriptor flags for working with the entity.
    /// * `root` - The root directory of the file system.
    /// * `path` - An iterator yielding `(Dir<T>, PathSegment)` tuples representing the directories
    ///   along the path to the entity.
    pub fn from_entity(
        entity: Entity<T>,
        name: Option<PathSegment>,
        flags: DescriptorFlags,
        root: RootDir<S>,
        path: impl IntoIterator<Item = (Dir<T>, PathSegment)>,
    ) -> Self {
        EntityHandle(Handle::from(entity, name, flags, root, path))
    }

    /// Creates a new handle from a file, its name, descriptor flags, root directory, and path.
    ///
    /// ## Arguments
    ///
    /// * `file` - The file being referenced by the handle.
    /// * `name` - The name of the entity in its parent directory entries. `None` if the handle has
    ///   no parent directory.
    /// * `flags` - The descriptor flags for working with the file.
    /// * `root` - The root directory of the file system.
    /// * `path` - An iterator yielding `(Dir<T>, PathSegment)` tuples representing the directories
    ///   along the path to the file.
    pub fn from_file(
        file: File<T>,
        name: Option<PathSegment>,
        flags: DescriptorFlags,
        root: RootDir<S>,
        pathdirs: impl IntoIterator<Item = (Dir<T>, PathSegment)>,
    ) -> Self {
        EntityHandle(Handle::from(
            Entity::File(file),
            name,
            flags,
            root,
            pathdirs,
        ))
    }

    /// Creates a new handle from a directory, its name, descriptor flags, root directory, and path.
    ///
    /// ## Arguments
    ///
    /// * `dir` - The directory being referenced by the handle.
    /// * `name` - The name of the directory in its parent directory entries. `None` if the handle has
    ///   no parent directory.
    /// * `flags` - The descriptor flags for working with the directory.
    /// * `root` - The root directory of the file system.
    /// * `path` - An iterator yielding `(Dir<T>, PathSegment)` tuples representing the directories
    ///   along the path to the directory.
    pub fn from_dir(
        dir: Dir<T>,
        name: Option<PathSegment>,
        flags: DescriptorFlags,
        root: RootDir<S>,
        path: impl IntoIterator<Item = (Dir<T>, PathSegment)>,
    ) -> Self {
        EntityHandle(Handle::from(Entity::Dir(dir), name, flags, root, path))
    }

    /// Tries to convert the handle to a file handle.
    pub fn as_file(self) -> FsResult<FileHandle<S, T>> {
        let EntityHandle(Handle {
            entity,
            name,
            flags,
            root,
            pathdirs,
        }) = self;

        entity
            .as_file()
            .map(|file| FileHandle::from(file, name, flags, root, pathdirs))
    }

    /// Tries to convert the handle to a directory handle.
    pub fn as_dir(self) -> FsResult<DirHandle<S, T>> {
        let EntityHandle(Handle {
            entity,
            name,
            flags,
            root,
            pathdirs,
        }) = self;

        entity
            .as_dir()
            .map(|dir| DirHandle::from(dir, name, flags, root, pathdirs))
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
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entity::File(file) => f.debug_tuple("File").field(file).finish(),
            Entity::Dir(dir) => f.debug_tuple("Dir").field(dir).finish(),
            Entity::Symlink(symlink) => f.debug_tuple("Symlink").field(symlink).finish(),
        }
    }
}

impl<S, T> Deref for EntityHandle<S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    type Target = Handle<Entity<T>, S, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
