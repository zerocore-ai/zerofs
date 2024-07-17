use std::{ops::Deref, sync::Arc};

use zeroutils_store::IpldStore;

use super::{DescriptorFlags, Dir, PathDirs, PathSegment, RootDir};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A handle represents an entity _available_ for reading and writing.
///
/// The handle not only contains the entity and its flags but also the root directory and path to
/// the entity which are necessary for content-addressable updates. i.e., when we make changes to
/// the entity and flush the changes, the parent alongs the entity's path should all contain the
/// correct [`Cids`][zeroutils_store::ipld::cid::Cid].
///
/// There are two stores here: `S` and `T` because there can be potentially different stores for
/// the root directory and path. The path would usually be backed by an ephemeral buffer store.
#[derive(Debug, Clone)]
pub struct Handle<E, S, T>
where
    S: IpldStore,
    T: IpldStore,
{
    inner: Arc<HandleInner<E, S, T>>,
}

#[derive(Debug, Clone)]
struct HandleInner<E, S, T>
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
            inner: Arc::new(HandleInner {
                entity,
                name,
                flags,
                root,
                pathdirs: pathdirs.into_iter().collect(),
            }),
        }
    }

    /// Returns the entity being referenced by the handle.
    pub fn entity(&self) -> &E {
        &self.inner.entity
    }

    /// Returns the name of the entity in its parent directory entries.
    pub fn name(&self) -> Option<&PathSegment> {
        self.inner.name.as_ref()
    }

    /// Returns the flags for the descriptor.
    pub fn flags(&self) -> &DescriptorFlags {
        &self.inner.flags
    }

    /// Returns the root directory.
    pub fn root(&self) -> RootDir<S> {
        self.inner.root.clone()
    }

    /// Returns the pathdirs to the entity.
    pub fn pathdirs(&self) -> &PathDirs<T> {
        &self.inner.pathdirs
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
        &self.inner.entity
    }
}
