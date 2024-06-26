use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use zeroutils_store::ipld::cid::Cid;

use crate::filesystem::{DescriptorFlags, OpenFlags, Path, PathFlags};

//--------------------------------------------------------------------------------------------------
// Types: Identifiers
//--------------------------------------------------------------------------------------------------

/// Represents an identifier that can be used by the service to identify the file system entity.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityIdentifier(Cid);

// pub enum StreamKind { Input, Output }
// pub struct StreamHandle { kind: StreamKind, handle: FileHandle }
// pub struct StreamOperation { pub handle: Option<StreamHandle>, pub operation_kind: StreamOperationKind, cap: Option<Ucan> }
// pub enum StreamOperationKind { Open, Chunk { data: Vec<u8> }, Close }

//--------------------------------------------------------------------------------------------------
// Types: Operations
//--------------------------------------------------------------------------------------------------

/// Represents an operation that can be performed on an entity.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityOperation {
    /// The identifier of the entity to perform the operation on.
    ///
    /// `None` when the operation is to be applied on the root directory the file tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntityIdentifier>,

    /// The operation to perform on the entity.
    pub operation: EntityOperationKind,
}

/// Represents an operation that can be performed on an entity.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "params", rename_all = "snake_case")]
pub enum EntityOperationKind {
    /// `Open` returns a handle to the entity that can be used to perform other operations on it.
    OpenAt(OpenAt),
}

/// Represents an operation that opens an entity at a given path.
#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAt {
    /// The path to the entity to open.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    path: Path,

    /// Flags that determine how the path is resolved and how the entity is opened.
    path_flags: PathFlags, // TODO: Should serialize to u8

    /// Flags that determine how the entity is opened.
    open_flags: OpenFlags, // TODO: Should serialize to u8

    /// Flags that deal with capabilities of the entity.
    descriptor_flags: DescriptorFlags, // TODO: Should serialize to u8
}
