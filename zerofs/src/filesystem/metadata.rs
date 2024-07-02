use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::EntityType;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Relevant metadata for a file system entity.
///
/// This mostly corresponds to the `descriptor-stat` structure in the WASI. `zerofs` does not support
/// hard links, so there is no `link-count` field. Also `size` is not stored here, but rather
/// requested when needed.
///
// TODO: Need to to know precisely what the DateTimes serialize to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    /// The type of the entity.
    pub entity_type: EntityType,

    /// The time the entity was created.
    pub created_at: DateTime<Utc>,

    /// The time of the last modification of the entity.
    pub modified_at: DateTime<Utc>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Metadata {
    /// Creates a new metadata object.
    pub fn new(entity_type: EntityType) -> Self {
        let now = Utc::now();

        Self {
            entity_type,
            created_at: now,
            modified_at: now,
        }
    }
}
