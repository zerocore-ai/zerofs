use std::{path::PathBuf, sync::Arc};

use tokio::sync::RwLock;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The maximum size of a block in the disk IPLD store.
// TODO: Not supported yet. In the future, we will use this to break big IPLD blocks into smaller blocks.
pub const DISK_IPLD_STORE_BLOCK_SIZE: usize = 256 * 1024; // 256 KiB

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A block store that stores blocks on disk.
#[derive(Clone)]
pub struct DiskStore {
    _inner: Arc<RwLock<DiskStoreInner>>,
}

struct DiskStoreInner {
    /// The base directory where the blocks are stored.
    ///
    /// Default is set to `~/.zerofs`.
    _base_dir: PathBuf,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl DiskStore {
    /// Creates a new `DiskStore` with the given base directory.
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            _inner: Arc::new(RwLock::new(DiskStoreInner {
                _base_dir: base_dir.into(),
            })),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

// TODO: Implement `IpldStore` for `DiskStore`.
