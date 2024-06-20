use std::time::SystemTime;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The kind of an entity in the file system.
///
/// This corresponds to `descriptor-type` in the WASI. `zerofs` does not support all the types that WASI
/// supports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityType {
    /// The entity is a regular file.
    File,
    /// The entity is a directory.
    Dir,
    /// The entity is a symbolic link.
    Symlink,
}

/// The kind of timestamp.
///
/// This corresponds to `new-timestamp` in the WASI.
pub enum TimestampType {
    /// Do not change the timestamp.
    NoChange,
    /// Set the timestamp to the current time.
    Now,
    /// Set the timestamp to the provided time.
    Timestamp(SystemTime),
}
