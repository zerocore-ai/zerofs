use super::{Dir, File};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// This is an entity in the file system.
pub enum Entity {
    /// A file.
    File(File),

    /// A directory.
    Dir(Dir),
    // /// A named pipe.
    // NamedPipe(NamedPipe),

    // /// A symbolic link.
    // Symlink(Symlink),

    // /// A block device.
    // BlockDevice(BlockDevice),
}
