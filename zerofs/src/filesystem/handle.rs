use std::ops::Deref;

use zeroutils_store::IpldStore;

use super::{DescriptorFlags, Dir, File};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A handle representing an open entity.
#[derive(Debug)]
pub struct Handle<E> {
    // ///
    // root: RootDir<S>,

    // ///
    // path: Vec<DirCidLink<S>>,

    // ///
    // tail: E,

    // ///
    // flags: DescriptorFlags,
    /// The entity.
    pub(crate) entity: E,

    /// The flags for the descriptor.
    pub(crate) flags: DescriptorFlags,
}

/// A handle for an open file.
pub type FileHandle<S> = Handle<File<S>>;

/// A handle for an open directory.
pub type DirHandle<S> = Handle<Dir<S>>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<E> Handle<E> {
    /// Creates a new handle.
    pub fn new(entity: E, flags: DescriptorFlags) -> Self {
        Handle { entity, flags }
    }

    /// Returns the flags for the descriptor.
    pub fn flags(&self) -> &DescriptorFlags {
        &self.flags
    }
}

impl<S> FileHandle<S>
where
    S: IpldStore,
{
    /// Creates a new file handle from a file and descriptor flags.
    pub fn from(entity: File<S>, flags: DescriptorFlags) -> FileHandle<S> {
        FileHandle { entity, flags }
    }
}

impl<S> DirHandle<S>
where
    S: IpldStore,
{
    /// Creates a new directory handle from a directory and descriptor flags.
    pub fn from(entity: Dir<S>, flags: DescriptorFlags) -> DirHandle<S> {
        DirHandle { entity, flags }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<E> Deref for Handle<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
