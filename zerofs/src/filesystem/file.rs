use core::fmt;
use std::{fmt::Debug, sync::Arc};

use serde::{
    de::{self, DeserializeSeed},
    Deserialize, Deserializer, Serialize, Serializer,
};
use zeroutils_store::{
    ipld::cid::Cid, IpldReferences, IpldStore, Storable, StoreError, StoreResult,
};

use super::{EntityFlags, EntityType, FileDescriptor, FsError, FsResult, Metadata};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A file in the file system.
#[derive(Clone)]
pub struct File<S>
where
    S: IpldStore,
{
    inner: Arc<FileInner<S>>,
}

struct FileInner<S>
where
    S: IpldStore,
{
    /// File metadata.
    pub(crate) metadata: Metadata,

    /// The store used to persist blocks in the file.
    pub(crate) store: S,
}

//--------------------------------------------------------------------------------------------------
// Types: Serializable
//--------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FileSerializable {
    metadata: Metadata,
}

pub(crate) struct FileDeserializeSeed<S> {
    pub(crate) store: S,
}

//--------------------------------------------------------------------------------------------------
// Methods: File
//--------------------------------------------------------------------------------------------------

impl<S> File<S>
where
    S: IpldStore,
{
    /// Creates a new file.
    pub fn new(store: S) -> Self {
        Self {
            inner: Arc::new(FileInner {
                metadata: Metadata::new(EntityType::File),
                store,
            }),
        }
    }

    /// Creates a new file descriptor for the file.
    pub fn into_fd(self, entity_flags: EntityFlags) -> FileDescriptor<S> {
        FileDescriptor {
            entity: self,
            flags: entity_flags,
        }
    }

    /// Deserializes to a `Dir` using an arbitrary deserializer and store.
    pub fn deserialize_with<'de>(
        deserializer: impl Deserializer<'de, Error: Into<FsError>>,
        store: S,
    ) -> FsResult<Self> {
        FileDeserializeSeed::new(store)
            .deserialize(deserializer)
            .map_err(Into::into)
    }

    /// Tries to create a new `Dir` from a serializable representation.
    pub(crate) fn try_from_serializable(
        serializable: FileSerializable,
        store: S,
    ) -> FsResult<Self> {
        Ok(File {
            inner: Arc::new(FileInner {
                metadata: serializable.metadata,
                store,
            }),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: FileDeserializeSeed
//--------------------------------------------------------------------------------------------------

impl<S> FileDeserializeSeed<S> {
    fn new(store: S) -> Self {
        Self { store }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S> IpldReferences for File<S>
where
    S: IpldStore,
{
    fn references<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Cid> + 'a> {
        // TODO: Fix when content is added to the file.
        Box::new(std::iter::empty())
    }
}

impl<S> Serialize for File<S>
where
    S: IpldStore,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let serializable = FileSerializable {
            metadata: self.inner.metadata.clone(),
        };

        serializable.serialize(serializer)
    }
}

impl<S> Storable<S> for File<S>
where
    S: IpldStore,
{
    async fn store(&self) -> StoreResult<Cid> {
        self.inner.store.put(self).await
    }

    async fn load(cid: &Cid, store: S) -> StoreResult<Self> {
        let serializable = store.get(cid).await?;
        File::try_from_serializable(serializable, store).map_err(StoreError::custom)
    }
}

impl<S> Debug for File<S>
where
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("metadata", &self.inner.metadata)
            .finish()
    }
}

impl<'de, S> DeserializeSeed<'de> for FileDeserializeSeed<S>
where
    S: IpldStore,
{
    type Value = File<S>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serializable = FileSerializable::deserialize(deserializer)?;
        File::try_from_serializable(serializable, self.store).map_err(de::Error::custom)
    }
}
