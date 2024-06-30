use std::ops::Deref;

use super::{DescriptorFlags, Dir, File};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A descriptor for an entity.
#[derive(Debug)]
pub struct Descriptor<E> {
    // ///
    // root: Arc<Mutex<Dir>>,

    // ///
    // spine: Vec<DirCidLink<S>>,

    // ///
    // tail: E,

    // ///
    // flags: DescriptorFlags,
    /// The entity.
    pub(crate) entity: E,

    /// The flags for the descriptor.
    pub(crate) flags: DescriptorFlags,
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
    pub fn new(entity: E, flags: DescriptorFlags) -> Self {
        Descriptor { entity, flags }
    }

    /// Returns the flags for the descriptor.
    pub fn flags(&self) -> &DescriptorFlags {
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
