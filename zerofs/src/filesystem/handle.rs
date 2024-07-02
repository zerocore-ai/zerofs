use std::ops::Deref;

use zeroutils_store::IpldStore;

use super::{DescriptorFlags, Dir, File, PathDirs, PathSegment, RootDir};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A handle represents an _opened_ entity.
///
/// The handle not only contains the entity and its flags but also the root directory and path to
/// the entity which are necessary for content-addressable updates. i.e., when we make changes to
/// the entity and flush the changes, the parent alongs the entity's path should all contain the
/// correct [`Cids`][zeroutils_store::ipld::cid::Cid].
///
/// There are two stores here: `S` and `T` because there can be potentially different stores for
/// the root directory and path. The path would usually be backed by an ephemeral buffer store.
#[derive(Debug)]
pub struct Handle<E, S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    /// The entity being referenced by the handle.
    pub(crate) entity: E,

    /// The name of the entity in its parent directory entries. `None` if the handle has no parent
    /// directory.
    pub(crate) name: Option<PathSegment>,

    /// The flags for working with the entity.
    pub(crate) flags: DescriptorFlags,

    /// The root directory of the file system.
    pub(crate) root: RootDir<S>,

    /// The directories along the path to the entity.
    pub(crate) pathdirs: PathDirs<T>,
}

/// A handle for an open file.
pub type FileHandle<S, T> = Handle<File<T>, S, T>;

/// A handle for an open directory.
pub type DirHandle<S, T> = Handle<Dir<T>, S, T>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<E, S, T> Handle<E, S, T>
where
    S: IpldStore,
    T: IpldStore,
{
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
    pub fn from(
        entity: E,
        name: Option<PathSegment>,
        flags: DescriptorFlags,
        root: RootDir<S>,
        pathdirs: impl IntoIterator<Item = (Dir<T>, PathSegment)>,
    ) -> Self {
        Handle {
            entity,
            name,
            flags,
            root,
            pathdirs: pathdirs.into_iter().collect(),
        }
    }

    /// Returns the name of the entity in its parent directory entries.
    pub fn name(&self) -> Option<&PathSegment> {
        self.name.as_ref()
    }

    /// Returns the flags for the descriptor.
    pub fn flags(&self) -> &DescriptorFlags {
        &self.flags
    }

    /// Returns the root directory.
    pub fn root(&self) -> RootDir<S> {
        self.root.clone()
    }

    /// Returns the pathdirs to the entity.
    pub fn pathdirs(&self) -> &PathDirs<T> {
        &self.pathdirs
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<E, S, T> Deref for Handle<E, S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
