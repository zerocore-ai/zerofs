use core::fmt;
use std::{fmt::Debug, sync::Arc};

use serde::{
    de::{self, DeserializeSeed},
    Deserialize, Deserializer, Serialize, Serializer,
};
use zeroutils_key::GetPublicKey;
use zeroutils_store::{
    ipld::cid::Cid, IpldReferences, IpldStore, Storable, StoreError, StoreResult,
};
use zeroutils_ucan::UcanAuth;

use super::{
    DescriptorFlags, EntityType, FileDescriptor, FileInputStream, FileOutputStream, FsError,
    FsResult, Metadata,
};

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

    /// File content. If the file is empty, this will be `None`.
    pub(crate) content: Option<Cid>,

    /// The store used to persist blocks in the file.
    pub(crate) store: S,
}

//--------------------------------------------------------------------------------------------------
// Types: Serializable
//--------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FileSerializable {
    metadata: Metadata,
    content: Option<Cid>,
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
                content: None,
                store,
            }),
        }
    }

    /// Creates a new file descriptor.
    pub fn new_descriptor(store: S, descriptor_flags: DescriptorFlags) -> FileDescriptor<S> {
        FileDescriptor {
            entity: File::new(store),
            flags: descriptor_flags,
        }
    }

    /// Creates a new file descriptor for the file.
    pub fn into_descriptor(self, flags: DescriptorFlags) -> FileDescriptor<S> {
        FileDescriptor {
            entity: self,
            flags,
        }
    }

    /// Returns the metadata for the directory.
    pub fn metadata(&self) -> &Metadata {
        &self.inner.metadata
    }

    /// Returns `true` if the file is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.content.is_none()
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
                content: serializable.content,
                store,
            }),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: FileDescriptor
//--------------------------------------------------------------------------------------------------

impl<S> FileDescriptor<S>
where
    S: IpldStore,
{
    /// Returns a stream to read from the file.
    pub fn read_via_stream<T, K>(
        &self,
        _offset: u64,
        _ucan: UcanAuth<T, K>,
    ) -> FsResult<FileInputStream<S>>
    where
        T: IpldStore,
        K: GetPublicKey,
    {
        todo!()
    }

    /// Returns a stream to write to the file.
    pub fn write_via_stream<T, K>(
        &self,
        _offset: u64,
        _ucan: UcanAuth<T, K>,
    ) -> FsResult<FileOutputStream<S>>
    where
        T: IpldStore,
        K: GetPublicKey,
    {
        todo!()
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
    fn references<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Cid> + Send + 'a> {
        match self.inner.content.as_ref() {
            Some(cid) => Box::new(std::iter::once(cid)),
            None => Box::new(std::iter::empty()),
        }
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
            content: self.inner.content,
        };

        serializable.serialize(serializer)
    }
}

impl<S> Storable<S> for File<S>
where
    S: IpldStore + Send + Sync,
{
    async fn store(&self) -> StoreResult<Cid> {
        self.inner.store.put_node(self).await
    }

    async fn load(cid: &Cid, store: S) -> StoreResult<Self> {
        let serializable = store.get_node(cid).await?;
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
