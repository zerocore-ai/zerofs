use std::{
    collections::{BTreeMap, HashMap},
    convert::{TryFrom, TryInto},
    fmt::{self, Debug},
    sync::{Arc, Mutex},
};

use serde::{
    de::{self, DeserializeSeed},
    Deserialize, Deserializer, Serialize, Serializer,
};
use zeroutils_store::{
    ipld::cid::Cid, IpldReferences, IpldStore, Storable, StoreError, StoreResult,
};

use crate::filesystem::{
    DescriptorFlags, Entity, EntityCidLink, EntityType, File, FsError, FsResult, Handle, Link,
    MemoryBufferStore, Metadata, Path, PathDirs, PathSegment, Resolvable,
};

//--------------------------------------------------------------------------------------------------
// Types: Dir
//--------------------------------------------------------------------------------------------------

/// Represents a directory node in the `zerofs` file system.
///
/// Since zerofs is a capability-based file system, a [`Ucan`][UcanAuth] needs to be provided that
/// lets the file system grant access to the directory's contents.
///
/// ## Important
///
/// Entities in `zerofs` are designed to be immutable and clone-on-write meaning writes create
/// forks of the entity.
#[derive(Clone)]
pub struct Dir<S>
where
    S: IpldStore,
{
    inner: Arc<DirInner<S>>,
}

#[derive(Clone)]
struct DirInner<S>
where
    S: IpldStore,
{
    /// Directory metadata.
    pub(crate) metadata: Metadata,

    /// The store used to persist blocks in the directory.
    pub(crate) store: S,

    /// The entries in the directory.
    pub(crate) entries: HashMap<PathSegment, EntityCidLink<S>>,
}

/// Used to represent the root directory of the file system.
///
/// Following the clone-on-write design of the file system, there is sometimes a need for a mutable
/// reference to the root directory of the file system. This is why the root directory is implemented
/// as an `Arc` and `Mutex`.
///
// TODO: Should probably consider actor-style model with channels.
#[derive(Debug, Clone)]
pub struct RootDir<S>
where
    S: IpldStore,
{
    inner: Arc<Mutex<Dir<S>>>,
}

/// A handle for an open directory.
pub type DirHandle<S, T> = Handle<Dir<T>, S, T>;

//--------------------------------------------------------------------------------------------------
// Types: *
//--------------------------------------------------------------------------------------------------

pub(crate) enum TraceResult<S>
where
    S: IpldStore,
{
    /// The entity was found.
    Found {
        /// The entity found.
        entity: Entity<S>,

        /// The name of the entity in its parent directory entries. `None` if the handle has
        /// no parent directory.
        name: Option<PathSegment>,

        /// The directories along the path to the entity.
        pathdirs: PathDirs<S>,
    },

    /// The entity was not found.
    Incomplete {
        /// The directories along the path to the entity.
        pathdirs: PathDirs<S>,

        /// The depth of the path to the entity.
        depth: usize,
    },

    /// Intermediate path is not a directory.
    NotADir {
        /// The directories along the path to the entity.
        pathdirs: PathDirs<S>,

        /// The depth of the path to the entity.
        depth: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DirSerializable {
    metadata: Metadata,
    entries: BTreeMap<String, Cid>,
}

pub(crate) struct DirDeserializeSeed<S> {
    pub(crate) store: S,
}

//--------------------------------------------------------------------------------------------------
// Methods: Dir
//--------------------------------------------------------------------------------------------------

impl<S> RootDir<S>
where
    S: IpldStore,
{
    /// Creates a new directory with the given store.
    pub fn new(store: S) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Dir::new(store))),
        }
    }

    /// Forks the root directory by creating a clone of it with an ephemeral buffer store.
    pub fn fork(&self) -> Dir<MemoryBufferStore<S>>
    where
        S: Send + Sync,
    {
        let dir = self.inner.lock().unwrap().clone();
        let buffer_store = MemoryBufferStore::new(dir.inner.store.clone());
        dir.use_store(buffer_store)
    }

    /// Creates a handle to the root directory with the given flags.
    pub fn make_handle(&self, flags: DescriptorFlags) -> DirHandle<S, MemoryBufferStore<S>>
    where
        S: Send + Sync,
    {
        DirHandle::from(self.fork(), None, flags, self.clone(), None)
    }
}

impl<S> Dir<S>
where
    S: IpldStore,
{
    /// Creates a new directory with the given store.
    pub fn new(store: S) -> Self {
        Self {
            inner: Arc::new(DirInner {
                metadata: Metadata::new(EntityType::Dir),
                entries: HashMap::new(),
                store,
            }),
        }
    }

    /// Adds a [`Cid`] (to an entity) and its associated name in the directory's entries.
    pub fn put(
        &mut self,
        name: impl TryInto<PathSegment, Error: Into<FsError>>,
        cid: Cid,
    ) -> FsResult<()> {
        let name = name.try_into().map_err(Into::into)?;
        let inner = Arc::make_mut(&mut self.inner);
        inner.entries.insert(name, EntityCidLink::from(cid));
        Ok(())
    }

    /// Gets the [`EntityCidLink`] with the given name from the directory's entries.
    pub fn get(&self, name: &PathSegment) -> Option<&EntityCidLink<S>> {
        self.inner.entries.get(name)
    }

    /// Returns the metadata for the directory.
    pub fn get_metadata(&self) -> &Metadata {
        &self.inner.metadata
    }

    /// Returns an iterator over the entries in the directory.
    pub fn get_entries(&self) -> impl Iterator<Item = (&PathSegment, &EntityCidLink<S>)> {
        self.inner.entries.iter()
    }

    /// Returns the store used to persist the file.
    pub fn get_store(&self) -> &S {
        &self.inner.store
    }

    /// Returns `true` if the directory is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.entries.is_empty()
    }

    /// Gets the entity with the provided name from the directory's entries, resolving it if necessary.
    pub async fn get_entity(&self, name: &PathSegment) -> FsResult<Option<&Entity<S>>>
    where
        S: Send + Sync,
    {
        match self.get(name) {
            Some(link) => Ok(Some(link.resolve(self.inner.store.clone()).await?)),
            None => Ok(None),
        }
    }

    /// Traces a given path to locate the target entity.
    ///
    /// This function navigates through the directory structure specified by `path`,
    /// resolving each segment until the final entity is found or an error occurs.
    ///
    /// ## Errors
    /// - `FsError::SymLinkNotSupportedYet`: Encountered a symbolic link, which is not supported.
    pub(crate) async fn trace_entity(&self, path: &Path) -> FsResult<TraceResult<S>>
    where
        S: Send + Sync,
    {
        let mut dir = self;
        let mut pathdirs = PathDirs::new();

        // First look up the intermediate directories except the last one.
        for (depth, segment) in path.slice(..path.len() - 1).iter().enumerate() {
            match dir.get_entity(segment).await? {
                Some(Entity::Dir(d)) => dir = d,
                Some(Entity::Symlink(_)) => {
                    return Err(FsError::SymLinkNotSupportedYet(
                        path.slice(0..depth).to_owned(),
                    ));
                }
                Some(_) => {
                    return Ok(TraceResult::NotADir { pathdirs, depth });
                }
                None => {
                    return Ok(TraceResult::Incomplete { pathdirs, depth });
                }
            }

            pathdirs.push((dir.clone(), segment.clone()));
        }

        // Then look up the last entity in the path.
        if let Some(segment) = path.last() {
            return match dir.get_entity(segment).await? {
                Some(entity) => Ok(TraceResult::Found {
                    entity: entity.clone(),
                    name: Some(segment.clone()),
                    pathdirs,
                }),
                None => Ok(TraceResult::Incomplete {
                    pathdirs,
                    depth: path.len(),
                }),
            };
        }

        Ok(TraceResult::Found {
            entity: Entity::Dir(dir.clone()),
            name: None,
            pathdirs,
        })
    }

    /// Retrieves an existing entity or creates a new one at the specified path.
    ///
    /// This function checks the existence of an entity at the given path. If the entity
    /// exists, it returns the entity and its corresponding path directories. If the
    /// entity does not exist, it creates a new directory hierarchy and returns the new
    /// entity and its corresponding path directories.
    ///
    /// `file` argument indicates whether to create a file (`true`) or a directory (`false`)
    /// if the entity does not exist.
    pub(crate) async fn get_or_create_entity(
        &self,
        path: &Path,
        file: bool,
    ) -> FsResult<(Entity<S>, Option<PathSegment>, PathDirs<S>)>
    where
        S: Send + Sync,
    {
        match self.trace_entity(path).await {
            Ok(TraceResult::Found {
                entity,
                name,
                pathdirs,
            }) => Ok((entity, name, pathdirs)),
            Ok(TraceResult::Incomplete {
                mut pathdirs,
                depth,
            }) => {
                for segment in path.slice(depth..path.len() - 1).iter() {
                    pathdirs.push((Dir::new(self.inner.store.clone()), segment.clone()));
                }

                let entity = if file {
                    Entity::File(File::new(self.inner.store.clone()))
                } else {
                    Entity::Dir(Dir::new(self.inner.store.clone()))
                };

                Ok((entity, path.last().cloned(), pathdirs))
            }
            Ok(TraceResult::NotADir { depth, .. }) => {
                Err(FsError::NotADirectory(Some(path.slice(..depth).to_owned())))
            }
            Err(e) => Err(e),
        }
    }

    /// Change the store used to persist the directory.
    pub fn use_store<T>(self, store: T) -> Dir<T>
    where
        T: IpldStore,
    {
        let inner = match Arc::try_unwrap(self.inner) {
            Ok(inner) => inner,
            Err(arc) => (*arc).clone(),
        };

        Dir {
            inner: Arc::new(DirInner {
                metadata: inner.metadata,
                entries: inner
                    .entries
                    .into_iter()
                    .map(|(k, v)| (k, v.use_store(&store)))
                    .collect(),
                store,
            }),
        }
    }

    /// Deserializes to a `Dir` using an arbitrary deserializer and store.
    pub fn deserialize_with<'de>(
        deserializer: impl Deserializer<'de, Error: Into<FsError>>,
        store: S,
    ) -> FsResult<Self> {
        DirDeserializeSeed::new(store)
            .deserialize(deserializer)
            .map_err(Into::into)
    }

    /// Tries to create a new `Dir` from a serializable representation.
    pub(crate) fn try_from_serializable(serializable: DirSerializable, store: S) -> FsResult<Self> {
        let entries: HashMap<_, _> = serializable
            .entries
            .into_iter()
            .map(|(segment, cid)| Ok((PathSegment::try_from(segment)?, Link::from(cid))))
            .collect::<FsResult<_>>()?;

        Ok(Dir {
            inner: Arc::new(DirInner {
                metadata: serializable.metadata,
                store,
                entries,
            }),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: DirDeserializeSeed
//--------------------------------------------------------------------------------------------------

impl<S> DirDeserializeSeed<S> {
    fn new(store: S) -> Self {
        Self { store }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<S> IpldReferences for Dir<S>
where
    S: IpldStore + Send + Sync,
{
    fn references<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Cid> + Send + 'a> {
        Box::new(self.get_entries().map(|(_, v)| v.get_cid()))
    }
}

impl<S> Storable<S> for Dir<S>
where
    S: IpldStore + Send + Sync,
{
    async fn store(&self) -> StoreResult<Cid> {
        self.inner.store.put_node(self).await
    }

    async fn load(cid: &Cid, store: S) -> StoreResult<Self> {
        let serializable: DirSerializable = store.get_node(cid).await?;
        Dir::try_from_serializable(serializable, store).map_err(StoreError::custom)
    }
}

impl<S> Debug for Dir<S>
where
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dir")
            .field("metadata", &self.inner.metadata)
            .field(
                "entries",
                &self
                    .get_entries()
                    .map(|(_, v)| v.get_cid())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl<S> Serialize for Dir<S>
where
    S: IpldStore,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let serializable = DirSerializable {
            metadata: self.inner.metadata.clone(),
            entries: self
                .get_entries()
                .map(|(k, v)| (k.to_string(), *v.get_cid()))
                .collect(),
        };

        serializable.serialize(serializer)
    }
}

impl<'de, S> DeserializeSeed<'de> for DirDeserializeSeed<S>
where
    S: IpldStore,
{
    type Value = Dir<S>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serializable = DirSerializable::deserialize(deserializer)?;
        Dir::try_from_serializable(serializable, self.store).map_err(de::Error::custom)
    }
}

impl<S> PartialEq for Dir<S>
where
    S: IpldStore,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<S> PartialEq for DirInner<S>
where
    S: IpldStore,
{
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
            && self.entries.len() == other.entries.len()
            && self.entries == other.entries
    }
}

impl<S> Debug for TraceResult<S>
where
    S: IpldStore,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceResult::Found {
                entity,
                name,
                pathdirs,
            } => f
                .debug_struct("Found")
                .field("entity", entity)
                .field("name", name)
                .field("pathdirs", pathdirs)
                .finish(),
            TraceResult::NotADir { pathdirs, depth } => f
                .debug_struct("NotADir")
                .field("pathdirs", pathdirs)
                .field("depth", depth)
                .finish(),
            TraceResult::Incomplete { pathdirs, depth } => f
                .debug_struct("Incomplete")
                .field("pathdirs", pathdirs)
                .field("depth", depth)
                .finish(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use zeroutils_store::MemoryStore;

    use super::*;

    #[tokio::test]
    async fn test_dir_constructor() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let dir = Dir::new(store);

        assert!(dir.inner.entries.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_dir_put_get_entries() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let mut dir = Dir::new(store);

        dir.put(
            "file1",
            "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq".parse()?,
        )?;

        dir.put(
            "file2",
            "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq".parse()?,
        )?;

        assert_eq!(dir.inner.entries.len(), 2);
        assert_eq!(
            dir.get(&"file1".parse()?).unwrap().get_cid(),
            &"bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq".parse()?
        );
        assert_eq!(
            dir.get(&"file2".parse()?).unwrap().get_cid(),
            &"bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq".parse()?
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_dir_stores_loads() -> anyhow::Result<()> {
        let store = MemoryStore::default();
        let mut dir = Dir::new(store.clone());

        dir.put(
            "file1",
            "bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq".parse()?,
        )?;

        let cid = dir.store().await?;
        let loaded_dir = Dir::load(&cid, store.clone()).await?;

        assert_eq!(dir, loaded_dir);

        Ok(())
    }
}
