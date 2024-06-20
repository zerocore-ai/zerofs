use cid::Cid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::filesystem::{EntityFlags, OpenFlags, Path, PathFlags};

//--------------------------------------------------------------------------------------------------
// Types: Handles
//--------------------------------------------------------------------------------------------------

/// Represents a handle to a file or directory in the file system.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityHandle {
    /// A handle to a file.
    File(FileHandle),
    /// A handle to a directory.
    Dir(DirHandle),
}

/// This is all the flags that can be set on an entity.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flags {
    /// Flags that determine how a path is resolved.
    pub path_flags: PathFlags,

    /// Flags that determine how a entity is opened.
    pub open_flags: OpenFlags,

    /// Flags that deal with capabilities of the entity.
    pub entity_flags: EntityFlags,
}

/// Represents a handle to a file in the file system.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileHandle {
    /// The CID that can be used to retrieve the file.
    pub id: Cid,

    /// Flags for working with the file.
    pub flags: Flags,
}

/// Represents a handle to a directory in the file system.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirHandle {
    /// The CID that can be used to retrieve the directory.
    pub id: Cid,

    /// Flags for working with the directory.
    pub flags: Flags,
}

// pub enum StreamKind { Input, Output }
// pub struct StreamHandle { kind: StreamKind, handle: FileHandle }
// pub struct StreamOperation { pub handle: Option<StreamHandle>, pub operation_kind: StreamOperationKind, cap: Option<Ucan> }
// pub enum StreamOperationKind { Open, Chunk { data: Vec<u8> }, Close }

//--------------------------------------------------------------------------------------------------
// Types: Operation
//--------------------------------------------------------------------------------------------------

/// Represents an operation that can be performed on an entity.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityOperation {
    /// The handle to the entity to perform the operation on.
    ///
    /// Note: This is `None` when the operation is to be applied to the root directory.
    pub handle: Option<EntityHandle>,

    /// The operation to perform on the entity.
    pub operation: EntityOperationKind,
}

/// Represents an operation that can be performed on an entity.
#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "params", rename_all = "snake_case")]
pub enum EntityOperationKind {
    /// `Open` returns a handle to the entity that can be used to perform other operations on it.
    OpenAt {
        /// The path to the entity to open.
        #[serde_as(as = "serde_with::DisplayFromStr")]
        path: Path,

        /// Flags that determine how the path is resolved and how the entity is opened.
        path_flags: PathFlags,

        /// Flags that determine how the entity is opened.
        open_flags: OpenFlags,

        /// Flags that deal with capabilities of the entity.
        entity_flags: EntityFlags,
    },
    /// `Read` returns an input stream that can be used to read the contents of the entity.
    Read {
        /// The path to the entity to read from.
        path: Path,

        /// The offset in the entity to start reading from.
        offset: u64,
    },
}
