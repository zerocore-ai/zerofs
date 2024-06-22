use std::{
    fmt::{self, Debug},
    sync::Arc,
};

use serde::{
    de::{self, DeserializeSeed},
    Deserialize, Deserializer, Serialize, Serializer,
};
use zeroutils_store::{
    ipld::cid::Cid, IpldReferences, IpldStore, Storable, StoreError, StoreResult,
};

use super::{EntityType, FsError, FsResult, Metadata, Path, PathLink};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A symlink to a file or directory.
#[derive(Clone)]
pub struct Symlink<S>
where
    S: IpldStore,
{
    inner: Arc<SymlinkInner<S>>,
}

struct SymlinkInner<S>
where
    S: IpldStore,
{
    /// The metadata of the symlink.
    pub(crate) metadata: Metadata,

    /// The store of the symlink.
    pub(crate) store: S,

    /// The link to the target of the symlink.
    pub(crate) link: PathLink<S>,
}

//--------------------------------------------------------------------------------------------------
// Types: Serializable
//--------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SymlinkSerializable {
    metadata: Metadata,
    link: Path,
}

pub(crate) struct SymlinkDeserializeSeed<S> {
    pub(crate) store: S,
}

//--------------------------------------------------------------------------------------------------
// Methods: Symlink
//--------------------------------------------------------------------------------------------------

impl<S> Symlink<S>
where
    S: IpldStore,
{
    /// Creates a new symlink.
    pub fn new(store: S, target: Path) -> Self {
        Self {
            inner: Arc::new(SymlinkInner {
                metadata: Metadata::new(EntityType::Symlink),
                store,
                link: PathLink::from(target),
            }),
        }
    }

    /// Gets the target path of the symlink.
    pub fn path(&self) -> &Path {
        self.inner.link.path()
    }

    /// Deserializes to a `Dir` using an arbitrary deserializer and store.
    pub fn deserialize_with<'de>(
        deserializer: impl Deserializer<'de, Error: Into<FsError>>,
        store: S,
    ) -> FsResult<Self> {
        SymlinkDeserializeSeed::new(store)
            .deserialize(deserializer)
            .map_err(Into::into)
    }

    /// Tries to create a new `Dir` from a serializable representation.
    pub(crate) fn try_from_serializable(
        serializable: SymlinkSerializable,
        store: S,
    ) -> FsResult<Self> {
        Ok(Symlink {
            inner: Arc::new(SymlinkInner {
                metadata: serializable.metadata,
                link: PathLink::from(serializable.link),
                store,
            }),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: FileDeserializeSeed
//--------------------------------------------------------------------------------------------------

impl<S> SymlinkDeserializeSeed<S> {
    fn new(store: S) -> Self {
        Self { store }
    }
}
//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S> IpldReferences for Symlink<S>
where
    S: IpldStore,
{
    fn references<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Cid> + 'a> {
        Box::new(std::iter::empty())
    }
}

impl<S> Serialize for Symlink<S>
where
    S: IpldStore,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let serializable = SymlinkSerializable {
            metadata: self.inner.metadata.clone(),
            link: self.inner.link.path().clone(),
        };

        serializable.serialize(serializer)
    }
}

impl<'de, S> DeserializeSeed<'de> for SymlinkDeserializeSeed<S>
where
    S: IpldStore,
{
    type Value = Symlink<S>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serializable = SymlinkSerializable::deserialize(deserializer)?;
        Symlink::try_from_serializable(serializable, self.store).map_err(de::Error::custom)
    }
}

impl<S> Storable<S> for Symlink<S>
where
    S: IpldStore,
{
    async fn store(&self) -> StoreResult<Cid> {
        self.inner.store.put(self).await
    }

    async fn load(cid: &Cid, store: S) -> StoreResult<Self> {
        let serializable = store.get(cid).await?;
        Symlink::try_from_serializable(serializable, store).map_err(StoreError::custom)
    }
}

impl<S> Debug for Symlink<S>
where
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Symlink")
            .field("metadata", &self.inner.metadata)
            .finish()
    }
}
