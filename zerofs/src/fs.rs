use crate::Dir;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A case-insensitive distributed file system.
pub struct FileSystem {
    /// The root directory of the file system.
    pub root: Dir,
}
