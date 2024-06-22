use std::ops::Deref;

use super::{Dir, EntityFlags, File};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A descriptor for an entity.
#[derive(Debug)]
pub struct Descriptor<E> {
    /// The entity.
    pub(crate) entity: E,

    /// The flags for the descriptor.
    pub(crate) flags: EntityFlags,
}

/// A descriptor for a file.
pub type FileDescriptor<S> = Descriptor<File<S>>;

/// A descriptor for a directory.
pub type DirDescriptor<S> = Descriptor<Dir<S>>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<E> Descriptor<E> {
    /// Creates a new descriptor.
    pub fn flags(&self) -> &EntityFlags {
        &self.flags
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<E> Deref for Descriptor<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
