use std::time::SystemTime;

use super::EntityType;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Relevant metadata for a file system entity.
///
/// This mostly corresponds to the `descriptor-stat` structure in the WASI.
/// `zerofs` does not support hard links, so there is no `link-count` field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// The type of the entity.
    pub entity_type: EntityType,

    /// The size of the entity in bytes.
    pub size: u64,

    /// The time of the last access of the entity.
    pub accessed_at: SystemTime,

    /// The time the entity was created.
    pub created_at: SystemTime,

    /// The time of the last modification of the entity.
    pub modified_at: SystemTime,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Metadata {
    /// Creates a new metadata object.
    pub fn new(entity_type: EntityType) -> Self {
        let now = SystemTime::now();

        Self {
            entity_type,
            size: 0,
            accessed_at: now,
            created_at: now,
            modified_at: now,
        }
    }
}
