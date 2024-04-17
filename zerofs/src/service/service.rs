use crate::{filesystem::FileSystem, BlockStore, ZerofsConfig};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A distributed service for the zerofs file system.
pub struct ZerofsService<S>
where
    S: BlockStore,
{
    // raft_node: MemRaftNode<FileOperations, OperationResponse>,
    _config: ZerofsConfig,
    _fs: FileSystem<S>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> ZerofsService<S> where S: BlockStore {}
