use crate::FsEntity;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A directory in the file system.
pub struct Dir {
    /// The name of the directory.
    pub name: String,

    /// The entries in the directory.
    pub entries: Vec<FsEntity>,
}
