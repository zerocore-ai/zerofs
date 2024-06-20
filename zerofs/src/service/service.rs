use std::sync::Arc;

use zeroutils_store::IpldStore;

use crate::{config::ZerofsConfig, filesystem::Dir};

use super::{FsServiceBuilder, ServiceResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A shared configuration for the file system service.
pub type SharedConfig = Arc<ZerofsConfig>;

/// `FsService` is a service that provides a distributed file system functionality.
///
/// This service uses a block store to store the file system data.
pub struct FsService<S>
where
    S: IpldStore,
{
    /// The root directory of the file system.
    pub root_dir: Dir<S>,

    /// The configuration of the file system.
    pub config: SharedConfig,

    // /// Raft node.
    // pub raft: RaftNode<FsStateMachine<DiskStore>, ...>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<S> FsService<S>
where
    S: IpldStore,
{
    /// Creates a new file system service with the given root directory and configuration.
    pub fn new(root_dir: Dir<S>, config: SharedConfig) -> Self {
        Self { root_dir, config }
    }

    /// Creates a file system builder.
    pub fn builder<'b>() -> FsServiceBuilder<'b> {
        FsServiceBuilder::default()
    }

    /// Starts the file system service.
    pub async fn start(&self) -> ServiceResult<()> {
        unimplemented!()
    }
}
