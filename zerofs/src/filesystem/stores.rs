use std::{collections::HashSet, path::PathBuf, pin::Pin, sync::Arc};

use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{io::AsyncRead, sync::RwLock};
use zeroutils_store::{
    ipld::cid::Cid, Codec, DualStore, DualStoreConfig, IpldReferences, IpldStore, MemoryStore,
    StoreResult,
};

//--------------------------------------------------------------------------------------------------
// Types: MemoryBufferStore
//--------------------------------------------------------------------------------------------------

/// An [`IpldStore`][zeroutils_store::IpldStore] with two underlying stores: an ephemeral in-memory
/// store for writes and a user-provided store for back-up reads.
///
/// This store is useful for creating a temporary buffer for writes
#[derive(Clone)]
pub struct MemoryBufferStore<S>
where
    S: IpldStore,
{
    inner: DualStore<MemoryStore, S>,
}

//--------------------------------------------------------------------------------------------------
// Types: DiskStore
//--------------------------------------------------------------------------------------------------

/// An [`IpldStore`][zeroutils_store::IpldStore] that stores its blocks on disk.
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
// Methods: MemoryBufferStore
//--------------------------------------------------------------------------------------------------

impl<S> MemoryBufferStore<S>
where
    S: IpldStore,
{
    /// Creates a new `MemoryBufferStore` with the given backup store.
    pub fn new(backup_store: S) -> Self {
        Self {
            inner: DualStore::new(
                MemoryStore::default(),
                backup_store,
                DualStoreConfig::default(),
            ),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: DiskStore
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

impl<S> IpldStore for MemoryBufferStore<S>
where
    S: IpldStore + Sync,
{
    async fn put_node<T>(&self, data: &T) -> StoreResult<Cid>
    where
        T: Serialize + IpldReferences + Sync,
    {
        self.inner.put_node(data).await
    }

    async fn put_bytes(&self, reader: impl AsyncRead + Send) -> StoreResult<Cid> {
        self.inner.put_bytes(reader).await
    }

    async fn put_raw_block(&self, bytes: impl Into<Bytes> + Send) -> StoreResult<Cid> {
        self.inner.put_raw_block(bytes).await
    }

    async fn get_node<T>(&self, cid: &Cid) -> StoreResult<T>
    where
        T: DeserializeOwned + Send,
    {
        self.inner.get_node(cid).await
    }

    async fn get_bytes<'a>(
        &'a self,
        cid: &'a Cid,
    ) -> StoreResult<Pin<Box<dyn AsyncRead + Send + 'a>>> {
        self.inner.get_bytes(cid).await
    }

    async fn get_raw_block(&self, cid: &Cid) -> StoreResult<Bytes> {
        self.inner.get_raw_block(cid).await
    }

    #[inline]
    async fn has(&self, cid: &Cid) -> bool {
        self.inner.has(cid).await
    }

    fn supported_codecs(&self) -> HashSet<Codec> {
        self.inner.supported_codecs()
    }

    #[inline]
    fn node_block_max_size(&self) -> Option<u64> {
        self.inner.node_block_max_size()
    }

    #[inline]
    fn raw_block_max_size(&self) -> Option<u64> {
        self.inner.raw_block_max_size()
    }
}
