use std::time::SystemTime;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Metadata for a file or directory in the file system.
pub struct Metadata {
    // pub mode: u32, // TODO: Replace with capability-based security model
    /// The size of the file or directory in bytes.
    pub size: u64,

    /// The creation time of the file or directory.
    pub created_at: SystemTime, // SystemTime?

    /// The last modified time of the file or directory.
    pub modified_at: SystemTime,

    /// The type of the entity.
    pub entity_type: EntityType,
}

/// The type of an entity in the file system.
pub enum EntityType {
    /// A file.
    File,

    /// A directory.
    Directory,
}
